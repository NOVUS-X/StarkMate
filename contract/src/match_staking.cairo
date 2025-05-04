use starknet::ContractAddress;

#[starknet::interface]
pub trait IMatchStaking<TContractState> {
    fn create_match(
        ref self: TContractState, match_id: felt252, player1: ContractAddress, wager_amount: u256,
    ) -> bool;
    fn join_match(ref self: TContractState, match_id: felt252, player2: ContractAddress) -> bool;
    fn cancel_match(ref self: TContractState, match_id: felt252) -> bool;
    fn claim_match(ref self: TContractState, match_id: felt252, winner: ContractAddress) -> bool;
    fn get_match_data(self: @TContractState, match_id: felt252) -> Match;
}

#[starknet::interface]
pub trait IERC20<TContractState> {
    fn transferFrom(
        ref self: TContractState, sender: ContractAddress, recipient: ContractAddress, amount: u256,
    ) -> bool;
    fn transfer(ref self: TContractState, recipient: ContractAddress, amount: u256) -> bool;
    fn balanceOf(self: @TContractState, account: ContractAddress) -> u256;
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub struct Match {
    pub player1: ContractAddress,
    pub player2: ContractAddress,
    pub wager_amount: u256,
    pub is_active: bool,
    pub is_completed: bool,
    pub token_address: ContractAddress,
}

#[starknet::contract]
pub mod MatchStaking {
    use core::traits::Into;
    use openzeppelin_access::ownable::OwnableComponent;
    use starknet::storage::{Map, StorageMapReadAccess, StorageMapWriteAccess};
    use starknet::{
        ContractAddress, contract_address_const, get_caller_address, get_contract_address,
    };
    use super::{IERC20Dispatcher, IERC20DispatcherTrait, Match};

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);

    // External
    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;

    // Internal
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        matches: Map<felt252, Match>,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MatchCreated {
        #[key]
        pub match_id: felt252,
        pub player1: ContractAddress,
        pub wager_amount: u256,
        pub token_address: ContractAddress,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MatchJoined {
        #[key]
        pub match_id: felt252,
        pub player2: ContractAddress,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MatchCancelled {
        #[key]
        pub match_id: felt252,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MatchClaimed {
        #[key]
        pub match_id: felt252,
        pub winner: ContractAddress,
        pub amount: u256,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        MatchCreated: MatchCreated,
        MatchJoined: MatchJoined,
        MatchCancelled: MatchCancelled,
        MatchClaimed: MatchClaimed,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
    }

    #[abi(embed_v0)]
    impl MatchStakingImpl of super::IMatchStaking<ContractState> {
        fn create_match(
            ref self: ContractState,
            match_id: felt252,
            player1: ContractAddress,
            wager_amount: u256,
        ) -> bool {
            // Only allow match creation by the player or contract owner
            let caller = get_caller_address();
            assert(
                caller == player1 || self.ownable.owner() == caller,
                'Caller must be player1 or owner',
            );

            // Ensure match doesn't already exist
            let existing_match = self.matches.read(match_id);
            assert(existing_match.is_active == false, 'Match already exists');

            // Create STRK token (or any ERC20) match
            let token_address: ContractAddress =
                0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d
                .try_into()
                .unwrap();

            // Create new match
            let new_match = Match {
                player1,
                player2: contract_address_const::<0>(),
                wager_amount,
                is_active: true,
                is_completed: false,
                token_address,
            };

            self.matches.write(match_id, new_match);

            // Transfer tokens from player1 to contract
            let contract_address = get_contract_address();
            let erc20 = IERC20Dispatcher { contract_address: token_address };
            let transfer_success = erc20.transferFrom(player1, contract_address, wager_amount);
            assert(transfer_success, 'Token transfer failed');

            // Emit event
            self.emit(MatchCreated { match_id, player1, wager_amount, token_address });

            true
        }

        fn join_match(
            ref self: ContractState, match_id: felt252, player2: ContractAddress,
        ) -> bool {
            // Only allow joining by the player2 or contract owner
            let caller = get_caller_address();
            assert(
                caller == player2 || self.ownable.owner() == caller,
                'Caller must be player2 or owner',
            );

            // Get existing match
            let match_data = self.matches.read(match_id);

            // Validate match state
            assert(match_data.is_active, 'Match not active');
            assert(match_data.is_completed == false, 'Match already completed');
            assert(match_data.player2 == contract_address_const::<0>(), 'Match already joined');
            assert(match_data.player1 != player2, 'Cannot join your own match');

            // Transfer tokens from player2 to contract
            let contract_address = get_contract_address();
            let erc20 = IERC20Dispatcher { contract_address: match_data.token_address };
            let transfer_success = erc20
                .transferFrom(player2, contract_address, match_data.wager_amount);
            assert(transfer_success, 'Token transfer failed');

            // Update match with player2
            let updated_match = Match {
                player1: match_data.player1,
                player2,
                wager_amount: match_data.wager_amount,
                is_active: match_data.is_active,
                is_completed: match_data.is_completed,
                token_address: match_data.token_address,
            };

            self.matches.write(match_id, updated_match);

            // Emit event
            self.emit(MatchJoined { match_id, player2 });

            true
        }

        fn cancel_match(ref self: ContractState, match_id: felt252) -> bool {
            // Get existing match
            let match_data = self.matches.read(match_id);

            // Validate match state
            assert(match_data.is_active, 'Match not active');
            assert(match_data.is_completed == false, 'Match already completed');

            // Only allow cancellation by player1, or player2 if joined, or owner
            let caller = get_caller_address();
            assert(
                caller == match_data.player1
                    || (caller == match_data.player2
                        && match_data.player2 != contract_address_const::<0>())
                    || self.ownable.owner() == caller,
                'Unauthorized',
            );

            // Return tokens to players
            let contract_address = get_contract_address();
            let erc20 = IERC20Dispatcher { contract_address: match_data.token_address };

            // Return tokens to player1
            let _ = erc20.transfer(match_data.player1, match_data.wager_amount);

            // Return tokens to player2 if they joined
            if (match_data.player2 != contract_address_const::<0>()) {
                let _ = erc20.transfer(match_data.player2, match_data.wager_amount);
            }

            // Update match state
            let updated_match = Match {
                player1: match_data.player1,
                player2: match_data.player2,
                wager_amount: match_data.wager_amount,
                is_active: false,
                is_completed: true,
                token_address: match_data.token_address,
            };

            self.matches.write(match_id, updated_match);

            // Emit event
            self.emit(MatchCancelled { match_id });

            true
        }

        fn claim_match(
            ref self: ContractState, match_id: felt252, winner: ContractAddress,
        ) -> bool {
            // Only allow claiming by the contract owner
            self.ownable.assert_only_owner();

            // Get existing match
            let match_data = self.matches.read(match_id);

            // Validate match state
            assert(match_data.is_active, 'Match not active');
            assert(match_data.is_completed == false, 'Match already completed');
            assert(match_data.player2 != contract_address_const::<0>(), 'Match not joined yet');

            // Verify winner is one of the players
            assert(
                winner == match_data.player1 || winner == match_data.player2,
                'Winner must be a match player',
            );

            // Calculate total prize (both wagers)
            let total_prize = match_data.wager_amount + match_data.wager_amount;

            // Transfer prize to winner
            let erc20 = IERC20Dispatcher { contract_address: match_data.token_address };
            let transfer_success = erc20.transfer(winner, total_prize);
            assert(transfer_success, 'Prize transfer failed');

            // Update match state
            let updated_match = Match {
                player1: match_data.player1,
                player2: match_data.player2,
                wager_amount: match_data.wager_amount,
                is_active: false,
                is_completed: true,
                token_address: match_data.token_address,
            };

            self.matches.write(match_id, updated_match);

            // Emit event
            self.emit(MatchClaimed { match_id, winner, amount: total_prize });

            true
        }

        fn get_match_data(self: @ContractState, match_id: felt252) -> Match {
            self.matches.read(match_id)
        }
    }
}
