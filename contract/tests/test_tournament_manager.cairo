use contract::tournament_manager::{
    ITournamentManagerDispatcher, ITournamentManagerDispatcherTrait, Tournament, TournamentStatus,
    BracketMatch, MatchStatus,
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
    
    // Mint tokens to test accounts
    let mock_erc20 = openzeppelin_token::erc20::interface::IERC20Dispatcher { 
        contract_address: token_address 
    };
    
    // We need to use the mint function from our MockERC20
    let mock_token_contract = starknet::syscalls::call_contract_syscall(
        token_address,
        selector!("mint"),
        array![ORGANIZER().into(), 10000_u256.low.into(), 10000_u256.high.into()].span()
    );
    
    let mock_token_contract = starknet::syscalls::call_contract_syscall(
        token_address,
        selector!("mint"),
        array![PLAYER1().into(), 1000_u256.low.into(), 1000_u256.high.into()].span()
    );
    
    let mock_token_contract = starknet::syscalls::call_contract_syscall(
        token_address,
        selector!("mint"),
        array![PLAYER2().into(), 1000_u256.low.into(), 1000_u256.high.into()].span()
    );
    
    (tournament_manager, token_address)
}

#[test]
fn test_create_tournament() {
    let (tournament_manager, token_address) = setup();
    
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);
    
    let tournament_id = tournament_manager.create_tournament(
        'Test Tournament',
        'A test tournament',
        2000, // start_time
        1500, // registration_deadline
        4,    // max_participants
        100,  // entry_fee
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
    
    stop_cheat_caller_address(tournament_manager.contract_address);
}

#[test]
fn test_player_registration() {
    let (tournament_manager, token_address) = setup();
    
    // Create tournament
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);
    
    let tournament_id = tournament_manager.create_tournament(
        'Test Tournament',
        'A test tournament',
        2000, // start_time
        1500, // registration_deadline
        4,    // max_participants
        100,  // entry_fee
        0,    // prize_pool (no initial prize pool)
        token_address,
    );
    
    stop_cheat_caller_address(tournament_manager.contract_address);
    
    // Approve tokens for entry fee
    let erc20 = openzeppelin_token::erc20::interface::IERC20Dispatcher { 
        contract_address: token_address 
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
        'Player should be registered'
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
    
    let tournament_id = tournament_manager.create_tournament(
        'Test Tournament',
        'A test tournament',
        2000, // start_time
        1500, // registration_deadline
        4,    // max_participants
        0,    // entry_fee (no fee for simplicity)
        1000, // prize_pool
        token_address,
    );
    
    // Register 4 players
    let players = array![PLAYER1(), PLAYER2(), PLAYER3(), PLAYER4()];
    let mut i = 0;
    while i < players.len() {
        let player = *players.at(i);
        start_cheat_caller_address(tournament_manager.contract_address, player);
        tournament_manager.register_player(tournament_id);
        stop_cheat_caller_address(tournament_manager.contract_address);
        i += 1;
    };
    
    // Check tournament is full and registration closed
    let tournament = tournament_manager.get_tournament(tournament_id);
    assert(tournament.current_participants == 4, 'Should have 4 participants');
    
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
    
    let tournament_id = tournament_manager.create_tournament(
        'Test Tournament',
        'A test tournament',
        2000,
        1500,
        4,
        0,
        1000,
        token_address,
    );
    
    // Register players
    let players = array![PLAYER1(), PLAYER2(), PLAYER3(), PLAYER4()];
    let mut i = 0;
    while i < players.len() {
        let player = *players.at(i);
        start_cheat_caller_address(tournament_manager.contract_address, player);
        tournament_manager.register_player(tournament_id);
        stop_cheat_caller_address(tournament_manager.contract_address);
        i += 1;
    };
    
    // Start tournament (this generates bracket)
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 2000);
    
    tournament_manager.start_tournament(tournament_id);
    
    // Check bracket was generated
    let bracket = tournament_manager.get_bracket(tournament_id);
    assert(bracket.len() == 2, 'Should have 2 first round matches');
    
    let current_round_matches = tournament_manager.get_current_round_matches(tournament_id);
    assert(current_round_matches.len() == 2, 'Should have 2 matches in current round');
    
    stop_cheat_caller_address(tournament_manager.contract_address);
}

#[test]
#[should_panic(expected: ('Participants must be power of 2',))]
fn test_invalid_participant_count() {
    let (tournament_manager, token_address) = setup();
    
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);
    
    // Try to create tournament with 3 participants (not power of 2)
    tournament_manager.create_tournament(
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
    
    let tournament_id = tournament_manager.create_tournament(
        'Test Tournament',
        'A test tournament',
        2000,
        1500,
        4,
        0,
        1000,
        token_address,
    );
    
    stop_cheat_caller_address(tournament_manager.contract_address);
    
    // Try to register after deadline
    start_cheat_caller_address(tournament_manager.contract_address, PLAYER1());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1600); // After deadline
    
    tournament_manager.register_player(tournament_id);
}

#[test]
fn test_tournament_cancellation() {
    let (tournament_manager, token_address) = setup();
    
    // Create tournament
    start_cheat_caller_address(tournament_manager.contract_address, ORGANIZER());
    start_cheat_block_timestamp(tournament_manager.contract_address, 1000);
    
    let tournament_id = tournament_manager.create_tournament(
        'Test Tournament',
        'A test tournament',
        2000,
        1500,
        4,
        0,
        1000,
        token_address,
    );
    
    // Cancel tournament
    tournament_manager.cancel_tournament(tournament_id);
    
    let tournament = tournament_manager.get_tournament(tournament_id);
    assert(tournament.status == TournamentStatus::Cancelled, 'Tournament should be cancelled');
    
    stop_cheat_caller_address(tournament_manager.contract_address);
}
