use starknet::ContractAddress;

#[starknet::interface]
pub trait ITournamentManager<TContractState> {
    // Tournament Management
    fn create_tournament(
        ref self: TContractState,
        name: felt252,
        description: felt252,
        start_time: u64,
        registration_deadline: u64,
        max_participants: u32,
        entry_fee: u256,
        prize_pool: u256,
        token_address: ContractAddress,
    ) -> u256;
    
    fn cancel_tournament(ref self: TContractState, tournament_id: u256);
    fn start_tournament(ref self: TContractState, tournament_id: u256);
    fn finalize_tournament(ref self: TContractState, tournament_id: u256);
    
    // Player Registration
    fn register_player(ref self: TContractState, tournament_id: u256);
    fn unregister_player(ref self: TContractState, tournament_id: u256);
    
    // Bracket Management
    fn generate_bracket(ref self: TContractState, tournament_id: u256);
    fn advance_round(ref self: TContractState, tournament_id: u256);
    fn report_match_result(
        ref self: TContractState,
        tournament_id: u256,
        match_id: u256,
        winner: ContractAddress,
    );
    
    // Prize Distribution
    fn distribute_prizes(ref self: TContractState, tournament_id: u256);
    fn claim_prize(ref self: TContractState, tournament_id: u256);
    
    // View Functions
    fn get_tournament(self: @TContractState, tournament_id: u256) -> Tournament;
    fn get_tournament_participants(self: @TContractState, tournament_id: u256) -> Array<ContractAddress>;
    fn get_bracket(self: @TContractState, tournament_id: u256) -> Array<BracketMatch>;
    fn get_current_round_matches(self: @TContractState, tournament_id: u256) -> Array<BracketMatch>;
    fn is_player_registered(self: @TContractState, tournament_id: u256, player: ContractAddress) -> bool;
    fn get_player_prize(self: @TContractState, tournament_id: u256, player: ContractAddress) -> u256;
    fn get_tournament_count(self: @TContractState) -> u256;
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub enum TournamentStatus {
    Created,
    RegistrationOpen,
    RegistrationClosed,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub enum MatchStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub struct Tournament {
    pub id: u256,
    pub name: felt252,
    pub description: felt252,
    pub organizer: ContractAddress,
    pub start_time: u64,
    pub registration_deadline: u64,
    pub max_participants: u32,
    pub current_participants: u32,
    pub entry_fee: u256,
    pub prize_pool: u256,
    pub token_address: ContractAddress,
    pub status: TournamentStatus,
    pub current_round: u32,
    pub total_rounds: u32,
    pub winner: ContractAddress,
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub struct BracketMatch {
    pub match_id: u256,
    pub tournament_id: u256,
    pub round: u32,
    pub position: u32,
    pub player1: ContractAddress,
    pub player2: ContractAddress,
    pub winner: ContractAddress,
    pub status: MatchStatus,
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub struct PrizeDistribution {
    pub position: u32,
    pub percentage: u32, // Basis points (10000 = 100%)
}

#[starknet::contract]
pub mod TournamentManager {
    use alexandria_math::pow;
    use core::array::ArrayTrait;
    use core::traits::Into;
    use openzeppelin_access::ownable::OwnableComponent;
    use starknet::storage::{
        Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess, Vec, VecTrait, MutableVecTrait,
    };
    use starknet::{ContractAddress, get_caller_address, get_block_timestamp, get_contract_address};
    use super::{
        Tournament, TournamentStatus, BracketMatch, MatchStatus, PrizeDistribution,
    };

    // Import ERC20 interface for prize handling
    #[starknet::interface]
    trait IERC20<TContractState> {
        fn transfer_from(
            ref self: TContractState,
            sender: ContractAddress,
            recipient: ContractAddress,
            amount: u256,
        ) -> bool;
        fn transfer(ref self: TContractState, recipient: ContractAddress, amount: u256) -> bool;
        fn balance_of(self: @TContractState, account: ContractAddress) -> u256;
    }

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);

    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        // Tournament data
        tournaments: Map<u256, Tournament>,
        tournament_count: u256,
        
        // Participant management
        tournament_participants: Map<(u256, u32), ContractAddress>, // (tournament_id, index) -> player
        player_tournament_index: Map<(u256, ContractAddress), u32>, // (tournament_id, player) -> index
        is_registered: Map<(u256, ContractAddress), bool>,
        
        // Bracket management
        tournament_matches: Map<(u256, u256), BracketMatch>, // (tournament_id, match_id) -> match
        tournament_match_count: Map<u256, u256>,
        round_matches: Map<(u256, u32), Vec<u256>>, // (tournament_id, round) -> match_ids
        
        // Prize management
        player_prizes: Map<(u256, ContractAddress), u256>,
        prize_claimed: Map<(u256, ContractAddress), bool>,
        
        // Default prize distribution (can be customized per tournament)
        default_prize_distribution: Map<u32, u32>, // position -> percentage (basis points)
        
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
    }

    // Events
    #[derive(Drop, starknet::Event)]
    pub struct TournamentCreated {
        #[key]
        pub tournament_id: u256,
        pub name: felt252,
        pub organizer: ContractAddress,
        pub start_time: u64,
        pub max_participants: u32,
        pub prize_pool: u256,
    }

    #[derive(Drop, starknet::Event)]
    pub struct PlayerRegistered {
        #[key]
        pub tournament_id: u256,
        #[key]
        pub player: ContractAddress,
        pub participant_count: u32,
    }

    #[derive(Drop, starknet::Event)]
    pub struct PlayerUnregistered {
        #[key]
        pub tournament_id: u256,
        #[key]
        pub player: ContractAddress,
        pub participant_count: u32,
    }

    #[derive(Drop, starknet::Event)]
    pub struct TournamentStarted {
        #[key]
        pub tournament_id: u256,
        pub participant_count: u32,
        pub total_rounds: u32,
    }

    #[derive(Drop, starknet::Event)]
    pub struct BracketGenerated {
        #[key]
        pub tournament_id: u256,
        pub total_matches: u256,
        pub rounds: u32,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MatchResultReported {
        #[key]
        pub tournament_id: u256,
        #[key]
        pub match_id: u256,
        pub winner: ContractAddress,
        pub round: u32,
    }

    #[derive(Drop, starknet::Event)]
    pub struct RoundAdvanced {
        #[key]
        pub tournament_id: u256,
        pub new_round: u32,
        pub matches_in_round: u32,
    }

    #[derive(Drop, starknet::Event)]
    pub struct TournamentCompleted {
        #[key]
        pub tournament_id: u256,
        pub winner: ContractAddress,
        pub prize_pool: u256,
    }

    #[derive(Drop, starknet::Event)]
    pub struct PrizeDistributed {
        #[key]
        pub tournament_id: u256,
        #[key]
        pub player: ContractAddress,
        pub amount: u256,
        pub position: u32,
    }

    #[derive(Drop, starknet::Event)]
    pub struct TournamentCancelled {
        #[key]
        pub tournament_id: u256,
        pub reason: felt252,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        TournamentCreated: TournamentCreated,
        PlayerRegistered: PlayerRegistered,
        PlayerUnregistered: PlayerUnregistered,
        TournamentStarted: TournamentStarted,
        BracketGenerated: BracketGenerated,
        MatchResultReported: MatchResultReported,
        RoundAdvanced: RoundAdvanced,
        TournamentCompleted: TournamentCompleted,
        PrizeDistributed: PrizeDistributed,
        TournamentCancelled: TournamentCancelled,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
        self.tournament_count.write(0);
        
        // Initialize default prize distribution
        // 1st place: 50%, 2nd place: 30%, 3rd place: 15%, 4th place: 5%
        self.default_prize_distribution.write(1, 5000); // 50%
        self.default_prize_distribution.write(2, 3000); // 30%
        self.default_prize_distribution.write(3, 1500); // 15%
        self.default_prize_distribution.write(4, 500);  // 5%
    }

    #[abi(embed_v0)]
    impl TournamentManagerImpl of super::ITournamentManager<ContractState> {
        fn create_tournament(
            ref self: ContractState,
            name: felt252,
            description: felt252,
            start_time: u64,
            registration_deadline: u64,
            max_participants: u32,
            entry_fee: u256,
            prize_pool: u256,
            token_address: ContractAddress,
        ) -> u256 {
            let caller = get_caller_address();
            let current_time = get_block_timestamp();
            
            // Validation
            assert(start_time > current_time, 'Start time must be in future');
            assert(registration_deadline < start_time, 'Registration must end before start');
            assert(registration_deadline > current_time, 'Registration deadline passed');
            assert(max_participants >= 2, 'Need at least 2 participants');
            assert(self._is_power_of_two(max_participants), 'Participants must be power of 2');
            
            let tournament_id = self.tournament_count.read() + 1;
            self.tournament_count.write(tournament_id);
            
            let tournament = Tournament {
                id: tournament_id,
                name,
                description,
                organizer: caller,
                start_time,
                registration_deadline,
                max_participants,
                current_participants: 0,
                entry_fee,
                prize_pool,
                token_address,
                status: TournamentStatus::RegistrationOpen,
                current_round: 0,
                total_rounds: self._calculate_rounds(max_participants),
                winner: 0.try_into().unwrap(),
            };
            
            self.tournaments.write(tournament_id, tournament);
            
            // If there's a prize pool, transfer tokens to contract
            if prize_pool > 0 {
                let erc20 = IERC20Dispatcher { contract_address: token_address };
                let success = erc20.transfer_from(caller, get_contract_address(), prize_pool);
                assert(success, 'Prize pool transfer failed');
            }
            
            self.emit(TournamentCreated {
                tournament_id,
                name,
                organizer: caller,
                start_time,
                max_participants,
                prize_pool,
            });
            
            tournament_id
        }

        fn cancel_tournament(ref self: ContractState, tournament_id: u256) {
            let mut tournament = self.tournaments.read(tournament_id);
            let caller = get_caller_address();
            
            // Only organizer or owner can cancel
            assert(
                caller == tournament.organizer || caller == self.ownable.owner(),
                'Unauthorized to cancel'
            );
            
            // Can only cancel before it starts
            assert(
                tournament.status == TournamentStatus::RegistrationOpen ||
                tournament.status == TournamentStatus::RegistrationClosed,
                'Cannot cancel started tournament'
            );
            
            tournament.status = TournamentStatus::Cancelled;
            self.tournaments.write(tournament_id, tournament);
            
            // Refund entry fees to participants
            self._refund_participants(tournament_id, tournament);
            
            // Refund prize pool to organizer
            if tournament.prize_pool > 0 {
                let erc20 = IERC20Dispatcher { contract_address: tournament.token_address };
                let _ = erc20.transfer(tournament.organizer, tournament.prize_pool);
            }
            
            self.emit(TournamentCancelled {
                tournament_id,
                reason: 'Cancelled by organizer'
            });
        }

        fn start_tournament(ref self: ContractState, tournament_id: u256) {
            let mut tournament = self.tournaments.read(tournament_id);
            let caller = get_caller_address();
            let current_time = get_block_timestamp();
            
            // Only organizer or owner can start
            assert(
                caller == tournament.organizer || caller == self.ownable.owner(),
                'Unauthorized to start'
            );
            
            assert(tournament.status == TournamentStatus::RegistrationClosed, 'Invalid status');
            assert(current_time >= tournament.start_time, 'Tournament not ready to start');
            assert(tournament.current_participants >= 2, 'Need at least 2 participants');
            
            tournament.status = TournamentStatus::InProgress;
            tournament.current_round = 1;
            self.tournaments.write(tournament_id, tournament);
            
            // Generate initial bracket
            self.generate_bracket(tournament_id);
            
            self.emit(TournamentStarted {
                tournament_id,
                participant_count: tournament.current_participants,
                total_rounds: tournament.total_rounds,
            });
        }

        fn register_player(ref self: ContractState, tournament_id: u256) {
            let mut tournament = self.tournaments.read(tournament_id);
            let caller = get_caller_address();
            let current_time = get_block_timestamp();
            
            // Validation
            assert(tournament.status == TournamentStatus::RegistrationOpen, 'Registration closed');
            assert(current_time <= tournament.registration_deadline, 'Registration deadline passed');
            assert(!self.is_registered.read((tournament_id, caller)), 'Already registered');
            assert(tournament.current_participants < tournament.max_participants, 'Tournament full');
            
            // Handle entry fee
            if tournament.entry_fee > 0 {
                let erc20 = IERC20Dispatcher { contract_address: tournament.token_address };
                let success = erc20.transfer_from(caller, get_contract_address(), tournament.entry_fee);
                assert(success, 'Entry fee payment failed');
                
                // Add entry fee to prize pool
                tournament.prize_pool += tournament.entry_fee;
            }
            
            // Register player
            let participant_index = tournament.current_participants;
            self.tournament_participants.write((tournament_id, participant_index), caller);
            self.player_tournament_index.write((tournament_id, caller), participant_index);
            self.is_registered.write((tournament_id, caller), true);
            
            tournament.current_participants += 1;
            
            // Close registration if full
            if tournament.current_participants == tournament.max_participants {
                tournament.status = TournamentStatus::RegistrationClosed;
            }
            
            self.tournaments.write(tournament_id, tournament);
            
            self.emit(PlayerRegistered {
                tournament_id,
                player: caller,
                participant_count: tournament.current_participants,
            });
        }

        fn unregister_player(ref self: ContractState, tournament_id: u256) {
            let mut tournament = self.tournaments.read(tournament_id);
            let caller = get_caller_address();
            let current_time = get_block_timestamp();
            
            // Validation
            assert(tournament.status == TournamentStatus::RegistrationOpen, 'Cannot unregister now');
            assert(current_time <= tournament.registration_deadline, 'Registration deadline passed');
            assert(self.is_registered.read((tournament_id, caller)), 'Not registered');
            
            // Refund entry fee
            if tournament.entry_fee > 0 {
                let erc20 = IERC20Dispatcher { contract_address: tournament.token_address };
                let _ = erc20.transfer(caller, tournament.entry_fee);
                tournament.prize_pool -= tournament.entry_fee;
            }
            
            // Remove player (simplified - in production, you'd want to compact the array)
            self.is_registered.write((tournament_id, caller), false);
            tournament.current_participants -= 1;
            
            // Reopen registration if it was closed
            if tournament.status == TournamentStatus::RegistrationClosed {
                tournament.status = TournamentStatus::RegistrationOpen;
            }
            
            self.tournaments.write(tournament_id, tournament);
            
            self.emit(PlayerUnregistered {
                tournament_id,
                player: caller,
                participant_count: tournament.current_participants,
            });
        }

        fn generate_bracket(ref self: ContractState, tournament_id: u256) {
            let tournament = self.tournaments.read(tournament_id);
            
            // Only organizer or owner can generate bracket
            let caller = get_caller_address();
            assert(
                caller == tournament.organizer || caller == self.ownable.owner(),
                'Unauthorized'
            );
            
            assert(tournament.status == TournamentStatus::InProgress, 'Tournament not in progress');
            assert(tournament.current_round == 1, 'Bracket already generated');
            
            let participants = tournament.current_participants;
            let matches_in_first_round = participants / 2;
            
            let mut match_id = 1_u256;
            let mut round_match_ids: Vec<u256> = VecTrait::new();
            
            // Generate first round matches
            let mut i = 0_u32;
            while i < matches_in_first_round {
                let player1 = self.tournament_participants.read((tournament_id, i * 2));
                let player2 = self.tournament_participants.read((tournament_id, i * 2 + 1));
                
                let bracket_match = BracketMatch {
                    match_id,
                    tournament_id,
                    round: 1,
                    position: i,
                    player1,
                    player2,
                    winner: 0.try_into().unwrap(),
                    status: MatchStatus::Pending,
                };
                
                self.tournament_matches.write((tournament_id, match_id), bracket_match);
                round_match_ids.append(match_id);
                
                match_id += 1;
                i += 1;
            };
            
            self.round_matches.write((tournament_id, 1), round_match_ids);
            self.tournament_match_count.write(tournament_id, match_id - 1);
            
            self.emit(BracketGenerated {
                tournament_id,
                total_matches: match_id - 1,
                rounds: tournament.total_rounds,
            });
        }

        fn report_match_result(
            ref self: ContractState,
            tournament_id: u256,
            match_id: u256,
            winner: ContractAddress,
        ) {
            let tournament = self.tournaments.read(tournament_id);
            let caller = get_caller_address();
            
            // Only organizer or owner can report results
            assert(
                caller == tournament.organizer || caller == self.ownable.owner(),
                'Unauthorized'
            );
            
            assert(tournament.status == TournamentStatus::InProgress, 'Tournament not in progress');
            
            let mut bracket_match = self.tournament_matches.read((tournament_id, match_id));
            assert(bracket_match.status == MatchStatus::Pending, 'Match already completed');
            assert(
                winner == bracket_match.player1 || winner == bracket_match.player2,
                'Invalid winner'
            );
            
            bracket_match.winner = winner;
            bracket_match.status = MatchStatus::Completed;
            self.tournament_matches.write((tournament_id, match_id), bracket_match);
            
            self.emit(MatchResultReported {
                tournament_id,
                match_id,
                winner,
                round: bracket_match.round,
            });
            
            // Check if round is complete
            self._check_round_completion(tournament_id);
        }

        fn advance_round(ref self: ContractState, tournament_id: u256) {
            let mut tournament = self.tournaments.read(tournament_id);
            let caller = get_caller_address();
            
            // Only organizer or owner can advance rounds
            assert(
                caller == tournament.organizer || caller == self.ownable.owner(),
                'Unauthorized'
            );
            
            assert(tournament.status == TournamentStatus::InProgress, 'Tournament not in progress');
            
            // Verify current round is complete
            assert(self._is_round_complete(tournament_id, tournament.current_round), 'Round not complete');
            
            if tournament.current_round == tournament.total_rounds {
                // Tournament is complete
                self.finalize_tournament(tournament_id);
                return;
            }
            
            // Advance to next round
            tournament.current_round += 1;
            self.tournaments.write(tournament_id, tournament);
            
            // Generate next round matches
            self._generate_next_round_matches(tournament_id, tournament.current_round);
            
            let next_round_matches = self.round_matches.read((tournament_id, tournament.current_round));
            
            self.emit(RoundAdvanced {
                tournament_id,
                new_round: tournament.current_round,
                matches_in_round: next_round_matches.len(),
            });
        }

        fn finalize_tournament(ref self: ContractState, tournament_id: u256) {
            let mut tournament = self.tournaments.read(tournament_id);
            let caller = get_caller_address();
            
            // Only organizer or owner can finalize
            assert(
                caller == tournament.organizer || caller == self.ownable.owner(),
                'Unauthorized'
            );
            
            assert(tournament.status == TournamentStatus::InProgress, 'Tournament not in progress');
            assert(self._is_round_complete(tournament_id, tournament.current_round), 'Tournament not complete');
            
            // Find the winner (winner of the final match)
            let final_round_matches = self.round_matches.read((tournament_id, tournament.current_round));
            assert(final_round_matches.len() == 1, 'Invalid final round');
            
            let final_match_id = final_round_matches.at(0);
            let final_match = self.tournament_matches.read((tournament_id, *final_match_id));
            
            tournament.winner = final_match.winner;
            tournament.status = TournamentStatus::Completed;
            self.tournaments.write(tournament_id, tournament);
            
            // Calculate and set prize distribution
            self._calculate_prize_distribution(tournament_id);
            
            self.emit(TournamentCompleted {
                tournament_id,
                winner: tournament.winner,
                prize_pool: tournament.prize_pool,
            });
        }

        fn distribute_prizes(ref self: ContractState, tournament_id: u256) {
            let tournament = self.tournaments.read(tournament_id);
            let caller = get_caller_address();
            
            // Only organizer or owner can distribute prizes
            assert(
                caller == tournament.organizer || caller == self.ownable.owner(),
                'Unauthorized'
            );
            
            assert(tournament.status == TournamentStatus::Completed, 'Tournament not completed');
            
            // Distribute prizes to top 4 players
            let erc20 = IERC20Dispatcher { contract_address: tournament.token_address };
            
            let mut position = 1_u32;
            while position <= 4 {
                let player = self._get_player_by_position(tournament_id, position);
                if player != 0.try_into().unwrap() {
                    let prize_amount = self.player_prizes.read((tournament_id, player));
                    if prize_amount > 0 && !self.prize_claimed.read((tournament_id, player)) {
                        let success = erc20.transfer(player, prize_amount);
                        if success {
                            self.prize_claimed.write((tournament_id, player), true);
                            
                            self.emit(PrizeDistributed {
                                tournament_id,
                                player,
                                amount: prize_amount,
                                position,
                            });
                        }
                    }
                }
                position += 1;
            };
        }

        fn claim_prize(ref self: ContractState, tournament_id: u256) {
            let tournament = self.tournaments.read(tournament_id);
            let caller = get_caller_address();
            
            assert(tournament.status == TournamentStatus::Completed, 'Tournament not completed');
            assert(!self.prize_claimed.read((tournament_id, caller)), 'Prize already claimed');
            
            let prize_amount = self.player_prizes.read((tournament_id, caller));
            assert(prize_amount > 0, 'No prize to claim');
            
            let erc20 = IERC20Dispatcher { contract_address: tournament.token_address };
            let success = erc20.transfer(caller, prize_amount);
            assert(success, 'Prize transfer failed');
            
            self.prize_claimed.write((tournament_id, caller), true);
            
            // Find player position for event
            let position = self._find_player_position(tournament_id, caller);
            
            self.emit(PrizeDistributed {
                tournament_id,
                player: caller,
                amount: prize_amount,
                position,
            });
        }

        // View Functions
        fn get_tournament(self: @ContractState, tournament_id: u256) -> Tournament {
            self.tournaments.read(tournament_id)
        }

        fn get_tournament_participants(self: @ContractState, tournament_id: u256) -> Array<ContractAddress> {
            let tournament = self.tournaments.read(tournament_id);
            let mut participants = ArrayTrait::new();
            
            let mut i = 0_u32;
            while i < tournament.current_participants {
                let player = self.tournament_participants.read((tournament_id, i));
                if self.is_registered.read((tournament_id, player)) {
                    participants.append(player);
                }
                i += 1;
            };
            
            participants
        }

        fn get_bracket(self: @ContractState, tournament_id: u256) -> Array<BracketMatch> {
            let mut matches = ArrayTrait::new();
            let match_count = self.tournament_match_count.read(tournament_id);
            
            let mut match_id = 1_u256;
            while match_id <= match_count {
                let bracket_match = self.tournament_matches.read((tournament_id, match_id));
                matches.append(bracket_match);
                match_id += 1;
            };
            
            matches
        }

        fn get_current_round_matches(self: @ContractState, tournament_id: u256) -> Array<BracketMatch> {
            let tournament = self.tournaments.read(tournament_id);
            let mut matches = ArrayTrait::new();
            
            if tournament.current_round > 0 {
                let round_match_ids = self.round_matches.read((tournament_id, tournament.current_round));
                let mut i = 0_u32;
                while i < round_match_ids.len() {
                    let match_id = round_match_ids.at(i);
                    let bracket_match = self.tournament_matches.read((tournament_id, *match_id));
                    matches.append(bracket_match);
                    i += 1;
                };
            }
            
            matches
        }

        fn is_player_registered(self: @ContractState, tournament_id: u256, player: ContractAddress) -> bool {
            self.is_registered.read((tournament_id, player))
        }

        fn get_player_prize(self: @ContractState, tournament_id: u256, player: ContractAddress) -> u256 {
            self.player_prizes.read((tournament_id, player))
        }

        fn get_tournament_count(self: @ContractState) -> u256 {
            self.tournament_count.read()
        }
    }

    // Internal helper functions
    #[generate_trait]
    impl InternalImpl of InternalTrait {
        fn _is_power_of_two(self: @ContractState, n: u32) -> bool {
            n > 0 && (n & (n - 1)) == 0
        }

        fn _calculate_rounds(self: @ContractState, participants: u32) -> u32 {
            let mut rounds = 0_u32;
            let mut n = participants;
            while n > 1 {
                n /= 2;
                rounds += 1;
            };
            rounds
        }

        fn _refund_participants(ref self: ContractState, tournament_id: u256, tournament: Tournament) {
            if tournament.entry_fee > 0 {
                let erc20 = IERC20Dispatcher { contract_address: tournament.token_address };
                
                let mut i = 0_u32;
                while i < tournament.current_participants {
                    let player = self.tournament_participants.read((tournament_id, i));
                    if self.is_registered.read((tournament_id, player)) {
                        let _ = erc20.transfer(player, tournament.entry_fee);
                    }
                    i += 1;
                };
            }
        }

        fn _is_round_complete(self: @ContractState, tournament_id: u256, round: u32) -> bool {
            let round_match_ids = self.round_matches.read((tournament_id, round));
            
            let mut i = 0_u32;
            while i < round_match_ids.len() {
                let match_id = round_match_ids.at(i);
                let bracket_match = self.tournament_matches.read((tournament_id, *match_id));
                if bracket_match.status != MatchStatus::Completed {
                    return false;
                }
                i += 1;
            };
            
            true
        }

        fn _check_round_completion(ref self: ContractState, tournament_id: u256) {
            let tournament = self.tournaments.read(tournament_id);
            
            if self._is_round_complete(tournament_id, tournament.current_round) {
                // Auto-advance if all matches in round are complete
                self.advance_round(tournament_id);
            }
        }

        fn _generate_next_round_matches(ref self: ContractState, tournament_id: u256, round: u32) {
            let previous_round_matches = self.round_matches.read((tournament_id, round - 1));
            let mut next_round_matches: Vec<u256> = VecTrait::new();
            
            let mut current_match_id = self.tournament_match_count.read(tournament_id) + 1;
            
            let mut i = 0_u32;
            while i < previous_round_matches.len() {
                if i + 1 < previous_round_matches.len() {
                    let match1_id = previous_round_matches.at(i);
                    let match2_id = previous_round_matches.at(i + 1);
                    
                    let match1 = self.tournament_matches.read((tournament_id, *match1_id));
                    let match2 = self.tournament_matches.read((tournament_id, *match2_id));
                    
                    let new_match = BracketMatch {
                        match_id: current_match_id,
                        tournament_id,
                        round,
                        position: i / 2,
                        player1: match1.winner,
                        player2: match2.winner,
                        winner: 0.try_into().unwrap(),
                        status: MatchStatus::Pending,
                    };
                    
                    self.tournament_matches.write((tournament_id, current_match_id), new_match);
                    next_round_matches.append(current_match_id);
                    
                    current_match_id += 1;
                }
                i += 2;
            };
            
            self.round_matches.write((tournament_id, round), next_round_matches);
            self.tournament_match_count.write(tournament_id, current_match_id - 1);
        }

        fn _calculate_prize_distribution(ref self: ContractState, tournament_id: u256) {
            let tournament = self.tournaments.read(tournament_id);
            
            // Get top 4 players based on tournament results
            let winner = tournament.winner;
            let runner_up = self._get_runner_up(tournament_id);
            let third_place = self._get_third_place(tournament_id);
            let fourth_place = self._get_fourth_place(tournament_id);
            
            // Distribute prizes based on default distribution
            if winner != 0.try_into().unwrap() {
                let prize = (tournament.prize_pool * self.default_prize_distribution.read(1).into()) / 10000;
                self.player_prizes.write((tournament_id, winner), prize);
            }
            
            if runner_up != 0.try_into().unwrap() {
                let prize = (tournament.prize_pool * self.default_prize_distribution.read(2).into()) / 10000;
                self.player_prizes.write((tournament_id, runner_up), prize);
            }
            
            if third_place != 0.try_into().unwrap() {
                let prize = (tournament.prize_pool * self.default_prize_distribution.read(3).into()) / 10000;
                self.player_prizes.write((tournament_id, third_place), prize);
            }
            
            if fourth_place != 0.try_into().unwrap() {
                let prize = (tournament.prize_pool * self.default_prize_distribution.read(4).into()) / 10000;
                self.player_prizes.write((tournament_id, fourth_place), prize);
            }
        }

        fn _get_runner_up(self: @ContractState, tournament_id: u256) -> ContractAddress {
            let tournament = self.tournaments.read(tournament_id);
            let final_round_matches = self.round_matches.read((tournament_id, tournament.total_rounds));
            
            if final_round_matches.len() > 0 {
                let final_match_id = final_round_matches.at(0);
                let final_match = self.tournament_matches.read((tournament_id, *final_match_id));
                
                if final_match.winner == final_match.player1 {
                    return final_match.player2;
                } else {
                    return final_match.player1;
                }
            }
            
            0.try_into().unwrap()
        }

        fn _get_third_place(self: @ContractState, tournament_id: u256) -> ContractAddress {
            // Simplified - in a real tournament, you'd track semifinal losers
            0.try_into().unwrap()
        }

        fn _get_fourth_place(self: @ContractState, tournament_id: u256) -> ContractAddress {
            // Simplified - in a real tournament, you'd track semifinal losers
            0.try_into().unwrap()
        }

        fn _get_player_by_position(self: @ContractState, tournament_id: u256, position: u32) -> ContractAddress {
            match position {
                1 => self.tournaments.read(tournament_id).winner,
                2 => self._get_runner_up(tournament_id),
                3 => self._get_third_place(tournament_id),
                4 => self._get_fourth_place(tournament_id),
                _ => 0.try_into().unwrap(),
            }
        }

        fn _find_player_position(self: @ContractState, tournament_id: u256, player: ContractAddress) -> u32 {
            let tournament = self.tournaments.read(tournament_id);
            
            if player == tournament.winner {
                return 1;
            }
            if player == self._get_runner_up(tournament_id) {
                return 2;
            }
            if player == self._get_third_place(tournament_id) {
                return 3;
            }
            if player == self._get_fourth_place(tournament_id) {
                return 4;
            }
            
            0
        }
    }
}
