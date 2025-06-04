use contract::tournament_manager::{
    BracketMatch, ITournamentManagerDispatcher, ITournamentManagerDispatcherTrait, MatchStatus,
    Tournament, TournamentStatus,
};
use core::array::ArrayTrait;
use openzeppelin_token::erc20::ERC20Component::InternalTrait;
use openzeppelin_token::erc20::{ERC20Component, ERC20HooksEmptyImpl};
use snforge_std::{
    ContractClassTrait, DeclareResultTrait, EventSpyAssertionsTrait, declare, spy_events,
    start_cheat_block_timestamp, start_cheat_caller_address, stop_cheat_caller_address,
};
use starknet::ContractAddress;

// Mock ERC20 for testing
#[starknet::contract]
mod MockERC20 {
    use openzeppelin_token::erc20::ERC20Component::InternalTrait;
    use openzeppelin_token::erc20::{ERC20Component, ERC20HooksEmptyImpl};
    use starknet::ContractAddress;

    component!(path: ERC20Component, storage: erc20, event: ERC20Event);

    impl ERC20HooksImpl = ERC20HooksEmptyImpl<ContractState>;

    #[abi(embed_v0)]
    impl ERC20Impl = ERC20Component::ERC20Impl<ContractState>;
    #[abi(embed_v0)]
    impl ERC20MetadataImpl = ERC20Component::ERC20MetadataImpl<ContractState>;
    impl InternalImpl = ERC20Component::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        erc20: ERC20Component::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ERC20Event: ERC20Component::Event,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        self.erc20.initializer("MockToken", "MTK");
    }

    #[external(v0)]
    fn mint(ref self: ContractState, recipient: ContractAddress, amount: u256) {
        self.erc20.mint(recipient, amount);
    }
}

fn OWNER() -> ContractAddress {
    'OWNER'.try_into().unwrap()
}

fn ORGANIZER() -> ContractAddress {
    'ORGANIZER'.try_into().unwrap()
}

fn PLAYER1() -> ContractAddress {
    'PLAYER1'.try_into().unwrap()
}

fn PLAYER2() -> ContractAddress {
    'PLAYER2'.try_into().unwrap()
}

fn PLAYER3() -> ContractAddress {
    'PLAYER3'.try_into().unwrap()
}

fn PLAYER4() -> ContractAddress {
    'PLAYER4'.try_into().unwrap()
}

fn deploy_mock_erc20() -> ContractAddress {
    let contract = declare("MockERC20").unwrap().contract_class();
    let calldata = ArrayTrait::new();
    let (address, _) = contract.deploy(@calldata).unwrap();
    address
}

fn deploy_tournament_manager(owner: ContractAddress) -> ITournamentManagerDispatcher {
    let contract = declare("TournamentManager").unwrap().contract_class();
    let calldata = array![owner.into()];
    let (address, _) = contract.deploy(@calldata).unwrap();
    ITournamentManagerDispatcher { contract_address: address }
}

fn setup() -> (ITournamentManagerDispatcher, ContractAddress) {
    let tournament_manager = deploy_tournament_manager(OWNER());
    let token_address = deploy_mock_erc20();

    // Create a dispatcher for the MockERC20 contract
    let mock_erc20_dispatcher = IMockERC20Dispatcher { contract_address: token_address };

    // Mint tokens to test accounts
    mock_erc20_dispatcher.mint(ORGANIZER(), 10000_u256);
    mock_erc20_dispatcher.mint(PLAYER1(), 1000_u256);
    mock_erc20_dispatcher.mint(PLAYER2(), 1000_u256);
    mock_erc20_dispatcher.mint(PLAYER3(), 1000_u256);
    mock_erc20_dispatcher.mint(PLAYER4(), 1000_u256);

    (tournament_manager, token_address)
}

// Add interface for MockERC20
#[starknet::interface]
trait IMockERC20<TContractState> {
    fn mint(ref self: TContractState, recipient: ContractAddress, amount: u256);
}

#[test]
fn test_create_tournament() {
    let (tournament_manager, token_address) = setup();

    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);

    let tournament_id = tournament_manager
        .create_tournament(
            'Test Tournament',
            'A test tournament',
            2000, // start_time
            1500, // registration_deadline
            4, // max_participants
            100, // entry_fee
            1000, // prize_pool
            token_address,
        );

    assert(tournament_id == 1, 'Tournament ID should be 1');

    let tournament = tournament_manager.get_tournament(tournament_id);
    assert(tournament.name == 'Test Tournament', 'Wrong tournament name');
    assert(tournament.organizer == ORGANIZER(), 'Wrong organizer');
    assert(tournament.max_participants == 4, 'Wrong max participants');
    assert(tournament.entry_fee == 100, 'Wrong entry fee');
    assert(tournament.prize_pool == 1000, 'Wrong prize pool');
    assert(tournament.status == TournamentStatus::RegistrationOpen, 'Wrong status');

    stop_cheat_caller_address(tournament_manager.contract_address);
}

#[test]
fn test_player_registration() {
    let (tournament_manager, token_address) = setup();

    // Create tournament
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);

    let tournament_id = tournament_manager
        .create_tournament(
            'Test Tournament',
            'A test tournament',
            2000, // start_time
            1500, // registration_deadline
            4, // max_participants
            100, // entry_fee
            0, // prize_pool (no initial prize pool)
            token_address,
        );

    stop_cheat_caller_address(tournament_manager.contract_address);

    // Approve tokens for entry fee
    let erc20 = openzeppelin_token::erc20::interface::IERC20Dispatcher {
        contract_address: token_address,
    };

    start_cheat_caller_address(token_address, PLAYER1());
    erc20.approve(tournament_manager.contract_address, 100);
    stop_cheat_caller_address(token_address);

    // Register player
    start_cheat_caller_address(tournament_manager.contract_address, PLAYER1());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1200);

    tournament_manager.register_player(tournament_id);

    assert(
        tournament_manager.is_player_registered(tournament_id, PLAYER1()),
        'Player should be registered',
    );

    let tournament = tournament_manager.get_tournament(tournament_id);
    assert(tournament.current_participants == 1, 'Should have 1 participant');
    assert(tournament.prize_pool == 100, 'Prize pool should include entry fee');

    stop_cheat_caller_address(tournament_manager.contract_address);
}

#[test]
fn test_tournament_lifecycle() {
    let (tournament_manager, token_address) = setup();

    // Create tournament
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);

    let tournament_id = tournament_manager
        .create_tournament(
            'Test Tournament',
            'A test tournament',
            2000, // start_time
            1500, // registration_deadline
            4, // max_participants
            0, // entry_fee (no fee for simplicity)
            1000, // prize_pool
            token_address,
        );

    // Register 4 players - need to set timestamp for each registration
    let players = array![PLAYER1(), PLAYER2(), PLAYER3(), PLAYER4()];
    let mut i = 0;
    while i < players.len() {
        let player = *players.at(i);
        start_cheat_caller_address(tournament_manager.contract_address, player);
        start_cheat_block_timestamp(tournament_manager.contract_address, 1200); // Before deadline
        tournament_manager.register_player(tournament_id);
        stop_cheat_caller_address(tournament_manager.contract_address);
        i += 1;
    }

    // Check tournament status after all players registered
    let tournament = tournament_manager.get_tournament(tournament_id);
    assert(tournament.current_participants == 4, 'Should have 4 participants');
    assert(
        tournament.status == TournamentStatus::RegistrationClosed, 'Should be registration closed',
    );

    // Start tournament
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 2000);

    tournament_manager.start_tournament(tournament_id);

    let tournament = tournament_manager.get_tournament(tournament_id);
    assert(tournament.status == TournamentStatus::InProgress, 'Tournament should be in progress');
    assert(tournament.current_round == 1, 'Should be in round 1');

    stop_cheat_caller_address(tournament_manager.contract_address);
}

#[test]
fn test_bracket_generation() {
    let (tournament_manager, token_address) = setup();

    // Create and setup tournament with 4 players
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);

    let tournament_id = tournament_manager
        .create_tournament(
            'Test Tournament', 'A test tournament', 2000, 1500, 4, 0, 1000, token_address,
        );

    // Register players
    let players = array![PLAYER1(), PLAYER2(), PLAYER3(), PLAYER4()];
    let mut i = 0;
    while i < players.len() {
        let player = *players.at(i);
        start_cheat_caller_address(tournament_manager.contract_address, player);
        start_cheat_block_timestamp(tournament_manager.contract_address, 1200);
        tournament_manager.register_player(tournament_id);
        stop_cheat_caller_address(tournament_manager.contract_address);
        i += 1;
    }

    // Start tournament (this generates bracket)
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 2000);

    tournament_manager.start_tournament(tournament_id);

    // Check bracket was generated
    let bracket = tournament_manager.get_bracket(tournament_id);
    assert(bracket.len() == 2, 'Should have 2 first round matches');

    let current_round_matches = tournament_manager.get_current_round_matches(tournament_id);
    assert(current_round_matches.len() == 2, 'Should have 2 matches in current round');

    // Verify match details
    let first_match = *bracket.at(0);
    assert(first_match.round == 1, 'Should be round 1');
    assert(first_match.status == MatchStatus::Pending, 'Should be pending');
    assert(first_match.player1 == PLAYER1(), 'Wrong player1');
    assert(first_match.player2 == PLAYER2(), 'Wrong player2');

    stop_cheat_caller_address(tournament_manager.contract_address);
}

#[test]
#[should_panic(expected: ('Participants must be power of 2',))]
fn test_invalid_participant_count() {
    let (tournament_manager, token_address) = setup();

    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);

    // Try to create tournament with 3 participants (not power of 2)
    tournament_manager
        .create_tournament(
            'Invalid Tournament',
            'Invalid tournament',
            2000,
            1500,
            3, // Invalid - not power of 2
            0,
            1000,
            token_address,
        );
}

#[test]
#[should_panic(expected: ('Registration deadline passed',))]
fn test_late_registration() {
    let (tournament_manager, token_address) = setup();

    // Create tournament
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);

    let tournament_id = tournament_manager
        .create_tournament(
            'Test Tournament', 'A test tournament', 2000, 1500, 4, 0, 1000, token_address,
        );

    stop_cheat_caller_address(tournament_manager.contract_address);

    // Try to register after deadline
    start_cheat_caller_address(tournament_manager.contract_address, PLAYER1());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1600); // After deadline (1500)

    tournament_manager.register_player(tournament_id);
}

#[test]
fn test_tournament_cancellation() {
    let (tournament_manager, token_address) = setup();

    // Create tournament
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);

    let tournament_id = tournament_manager
        .create_tournament(
            'Test Tournament', 'A test tournament', 2000, 1500, 4, 0, 1000, token_address,
        );

    // Cancel tournament
    tournament_manager.cancel_tournament(tournament_id);

    let tournament = tournament_manager.get_tournament(tournament_id);
    assert(tournament.status == TournamentStatus::Cancelled, 'Tournament should be cancelled');

    stop_cheat_caller_address(tournament_manager.contract_address);
}

#[test]
fn test_complete_match_flow() {
    let (tournament_manager, token_address) = setup();

    // Setup tournament with 4 players
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);

    let tournament_id = tournament_manager
        .create_tournament(
            'Test Tournament', 'A test tournament', 2000, 1500, 4, 0, 1000, token_address,
        );

    // Register players
    let players = array![PLAYER1(), PLAYER2(), PLAYER3(), PLAYER4()];
    let mut i = 0;
    while i < players.len() {
        let player = *players.at(i);
        start_cheat_caller_address(tournament_manager.contract_address, player);
        start_cheat_block_timestamp(tournament_manager.contract_address, 1200);
        tournament_manager.register_player(tournament_id);
        stop_cheat_caller_address(tournament_manager.contract_address);
        i += 1;
    }

    // Start tournament
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 2000);

    tournament_manager.start_tournament(tournament_id);

    // Get first round matches
    let matches = tournament_manager.get_current_round_matches(tournament_id);
    assert(matches.len() == 2, 'Should have 2 matches in round 1');

    // Report results for both matches
    let match1 = *matches.at(0);
    let match2 = *matches.at(1);

    tournament_manager.report_match_result(tournament_id, match1.match_id, match1.player1);
    tournament_manager.report_match_result(tournament_id, match2.match_id, match2.player1);

    // Tournament should auto-advance or we need to manually advance
    // Check if tournament advanced to final
    let tournament_after_round1 = tournament_manager.get_tournament(tournament_id);

    // If auto-advance is working, tournament should be in round 2 or completed
    // If not, we need to manually advance
    if tournament_after_round1.current_round == 1 {
        tournament_manager.advance_round(tournament_id);
    }

    let tournament = tournament_manager.get_tournament(tournament_id);
    assert(
        tournament.current_round >= 2 || tournament.status == TournamentStatus::Completed,
        'Should advance to next round or complete',
    );

    stop_cheat_caller_address(tournament_manager.contract_address);
}
