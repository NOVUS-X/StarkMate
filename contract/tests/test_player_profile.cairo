use starknet::ContractAddress;
use starknet::testing::{set_caller_address, set_contract_address};
use player_profile::player_profile::{IPlayerProfileDispatcher, IPlayerProfileDispatcherTrait};
use player_profile::player_profile::{PlayerProfile, PlayerStats};

fn deploy() -> IPlayerProfileDispatcher {
    let contract_address = starknet::contract_address_const::<0x123>();
    set_contract_address(contract_address);
    IPlayerProfileDispatcher { contract_address }
}

#[test]
fn test_create_profile() {
    let contract = deploy();
    let player_address = starknet::contract_address_const::<0x456>();
    let username = 'player1';

    // Set caller as the player
    set_caller_address(player_address);

    // Create profile
    contract.create_profile(player_address, username);

    // Verify profile was created correctly
    let profile = contract.get_profile(player_address);
    assert(profile.address == player_address, 'Wrong address');
    assert(profile.username == username, 'Wrong username');
    assert(profile.is_active == true, 'Profile not active');

    // Verify initial stats
    let stats = profile.stats;
    assert(stats.games_played == 0, 'Wrong games played');
    assert(stats.wins == 0, 'Wrong wins');
    assert(stats.losses == 0, 'Wrong losses');
    assert(stats.ranking == 0, 'Wrong ranking');
}

#[test]
#[should_panic(expected: ('Profile already exists', ))]
fn test_create_duplicate_profile() {
    let contract = deploy();
    let player_address = starknet::contract_address_const::<0x456>();
    let username = 'player1';

    // Set caller as the player
    set_caller_address(player_address);

    // Create profile first time
    contract.create_profile(player_address, username);

    // Try to create profile again
    contract.create_profile(player_address, username);
}

#[test]
#[should_panic(expected: ('Username already taken', ))]
fn test_create_profile_duplicate_username() {
    let contract = deploy();
    let player1_address = starknet::contract_address_const::<0x456>();
    let player2_address = starknet::contract_address_const::<0x789>();
    let username = 'player1';

    // Create profile for player1
    set_caller_address(player1_address);
    contract.create_profile(player1_address, username);

    // Try to create profile for player2 with same username
    set_caller_address(player2_address);
    contract.create_profile(player2_address, username);
}

#[test]
#[should_panic(expected: ('Only address owner can create profile', ))]
fn test_create_profile_wrong_caller() {
    let contract = deploy();
    let player_address = starknet::contract_address_const::<0x456>();
    let wrong_caller = starknet::contract_address_const::<0x789>();
    let username = 'player1';

    // Set wrong caller
    set_caller_address(wrong_caller);

    // Try to create profile
    contract.create_profile(player_address, username);
}

#[test]
fn test_update_stats() {
    let contract = deploy();
    let player_address = starknet::contract_address_const::<0x456>();
    let username = 'player1';

    // Create profile
    set_caller_address(player_address);
    contract.create_profile(player_address, username);

    // Update stats with a win
    contract.update_stats(player_address, 1, 0);

    // Verify stats
    let stats = contract.get_stats(player_address);
    assert(stats.games_played == 1, 'Wrong games played');
    assert(stats.wins == 1, 'Wrong wins');
    assert(stats.losses == 0, 'Wrong losses');
    assert(stats.ranking == 100, 'Wrong ranking');

    // Update stats with a loss
    contract.update_stats(player_address, 0, 1);

    // Verify updated stats
    let stats = contract.get_stats(player_address);
    assert(stats.games_played == 2, 'Wrong games played');
    assert(stats.wins == 1, 'Wrong wins');
    assert(stats.losses == 1, 'Wrong losses');
    assert(stats.ranking == 50, 'Wrong ranking');

    // Update stats with multiple wins and losses
    contract.update_stats(player_address, 2, 1);

    // Verify final stats
    let stats = contract.get_stats(player_address);
    assert(stats.games_played == 5, 'Wrong games played');
    assert(stats.wins == 3, 'Wrong wins');
    assert(stats.losses == 2, 'Wrong losses');
    assert(stats.ranking == 60, 'Wrong ranking');
}

#[test]
#[should_panic(expected: ('Profile does not exist', ))]
fn test_update_stats_nonexistent_profile() {
    let contract = deploy();
    let nonexistent_address = starknet::contract_address_const::<0x456>();

    // Try to update stats for nonexistent profile
    contract.update_stats(nonexistent_address, 1, 0);
}

#[test]
#[should_panic(expected: ('Profile does not exist', ))]
fn test_get_profile_nonexistent() {
    let contract = deploy();
    let nonexistent_address = starknet::contract_address_const::<0x456>();

    // Try to get nonexistent profile
    contract.get_profile(nonexistent_address);
} 