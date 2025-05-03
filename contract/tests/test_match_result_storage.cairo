use contract::match_result_storage::{
    IMatchResultStorageDispatcher, IMatchResultStorageDispatcherTrait, MatchResultStorage, MatchResult, MatchResultStorage::MatchResultStored
};
use starknet::{ContractAddress};
use snforge_std::{declare, ContractClassTrait, DeclareResultTrait, start_cheat_caller_address, start_cheat_block_timestamp, stop_cheat_caller_address, spy_events, EventSpyAssertionsTrait};
use core::array::ArrayTrait;
use core::result::ResultTrait;
use core::num::traits::Zero;

fn ORACLE() -> ContractAddress {
    'ORACLE'.try_into().unwrap()
}

fn PLAYER_WHITE() -> ContractAddress {
    'PLAYER_WHITE'.try_into().unwrap()
}

fn PLAYER_BLACK() -> ContractAddress {
    'PLAYER_BLACK'.try_into().unwrap()
}

// Helper function to deploy a StoreResult contract
fn deploy_store_result(
    oracle_address: ContractAddress
) -> ContractAddress {
    let contract = declare("MatchResultStorage").unwrap().contract_class();

    let mut constructor_calldata = ArrayTrait::new();
    oracle_address.serialize(ref constructor_calldata);

    let (contract_address, _) = contract.deploy(@constructor_calldata).unwrap();
    contract_address
}

#[test]
fn test_setup() {
    let store_result_address = deploy_store_result(ORACLE());

    let storeresult = IMatchResultStorageDispatcher { contract_address: store_result_address };

    assert(
        store_result_address !=  0.try_into().unwrap(), 
        'Contract deployed'
    );
    assert!(storeresult.get_oracle_address() == ORACLE(), "Incorrect oracle address");
}


#[test]
fn test_store_and_get_result() {
    // Define oracle and player addresses
    let match_id: felt252 = 100;
    let result: felt252 = 'WhiteWin';

    // Deploy the contract with the oracle address
    let store_result_address = deploy_store_result(ORACLE());
    let store_result_dispatcher = IMatchResultStorageDispatcher { contract_address: store_result_address }; 

    // Setup event spy
    let mut spy = spy_events();

    // Cheat the caller address to be the oracle and set a block timestamp
    start_cheat_caller_address(store_result_dispatcher.contract_address, ORACLE());
    let cheated_timestamp: u64 = 1678886400; // Example timestamp
    start_cheat_block_timestamp(store_result_dispatcher.contract_address, cheated_timestamp);

    // Store the match result
    store_result_dispatcher.store_result(match_id, PLAYER_WHITE(), PLAYER_BLACK(), result);

    // Stop cheating caller address
    stop_cheat_caller_address(store_result_dispatcher.contract_address);

    // Verify the result was stored correctly by retrieving it
    let stored_result: MatchResult = store_result_dispatcher.get_result(match_id);
    assert(stored_result.match_id == match_id, 'Wrong match_id');
    assert(stored_result.player_white == PLAYER_WHITE(), 'Wrong player_white');
    assert(stored_result.player_black == PLAYER_BLACK(), 'Wrong player_black');
    assert(stored_result.result == result, 'Wrong result');
    assert(stored_result.timestamp == cheated_timestamp, 'Wrong timestamp');

    // Verify event emission
    let expected_event = MatchResultStorage::Event::MatchResultStored(
        MatchResultStored {
            match_id,
            player_white: PLAYER_WHITE(),
            player_black: PLAYER_BLACK(),
            result: result,
            timestamp: cheated_timestamp,
        }
    );
    let expected_events = array![(store_result_dispatcher.contract_address, expected_event)];
    spy.assert_emitted(@expected_events);
}

#[test]
#[should_panic(expected: 'Only oracle can store result')]
fn test_store_result_unauthorized() {
    let unauthorized_caller: ContractAddress = 999.try_into().unwrap();
    let match_id: felt252 = 101;
    let result: felt252 = 'Draw';

    let store_result_address = deploy_store_result(ORACLE());
    let store_result_dispatcher = IMatchResultStorageDispatcher { contract_address: store_result_address }; 

    // Cheat the caller address to be the unauthorized address
    start_cheat_caller_address(store_result_dispatcher.contract_address, unauthorized_caller);

    // Attempt to store the match result (should panic)
    store_result_dispatcher.store_result(match_id, PLAYER_WHITE(), PLAYER_BLACK(), result);

    // Stop cheating caller address
    stop_cheat_caller_address(store_result_dispatcher.contract_address);
}

#[test]
fn test_get_nonexistent_result() {
    let store_result_address = deploy_store_result(ORACLE());
    let store_result_dispatcher = IMatchResultStorageDispatcher { contract_address: store_result_address }; 

    let nonexistent_match_id: felt252 = 999;

    let stored_result: MatchResult = store_result_dispatcher.get_result(nonexistent_match_id);

    assert(stored_result.match_id.is_zero(), 'match_id should be zero');
    assert(stored_result.player_white.is_zero(), 'player_white should be zero');
    assert(stored_result.player_black.is_zero(), 'player_black should be zero');
    assert(stored_result.result.is_zero(), 'result should be zero');
    assert(stored_result.timestamp.is_zero(), 'timestamp should be zero');
}