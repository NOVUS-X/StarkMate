use core::array::ArrayTrait;
use snforge_std::{ContractClassTrait, DeclareResultTrait, declare};
use starknet::ContractAddress;

/// Helper function to deploy the HelloStarknet contract
fn deploy_hello_contract() -> ContractAddress {
    let contract = declare("HelloStarknet").unwrap().contract_class();
    let calldata = ArrayTrait::new();
    let (contract_address, _) = contract.deploy(@calldata).unwrap();
    contract_address
}

/// Helper function to call get_balance
fn get_balance(contract_address: ContractAddress) -> felt252 {
    let selector = selector!("get_balance");
    let calldata = ArrayTrait::new();

    let result = starknet::syscalls::call_contract_syscall(
        contract_address, selector, calldata.span(),
    )
        .unwrap();

    *result.at(0)
}

/// Helper function to call increase_balance
fn increase_balance(contract_address: ContractAddress, amount: felt252) {
    let selector = selector!("increase_balance");
    let mut calldata = ArrayTrait::new();
    calldata.append(amount);

    let _ = starknet::syscalls::call_contract_syscall(contract_address, selector, calldata.span())
        .unwrap();
}

#[test]
fn test_initial_balance() {
    // Deploy the contract
    let contract_address = deploy_hello_contract();

    // Get the initial balance
    let balance = get_balance(contract_address);

    // Assert that the initial balance is 0
    assert(balance == 0, 'Initial balance should be 0');
}

#[test]
fn test_increase_balance() {
    // Deploy the contract
    let contract_address = deploy_hello_contract();

    // Get the initial balance
    let initial_balance = get_balance(contract_address);

    // Increase the balance by 42
    increase_balance(contract_address, 42);

    // Get the updated balance
    let updated_balance = get_balance(contract_address);

    // Assert that the balance increased correctly
    assert(updated_balance == initial_balance + 42, 'Balance should increase');
}

#[test]
fn test_multiple_increases() {
    // Deploy the contract
    let contract_address = deploy_hello_contract();

    // Increase the balance multiple times
    increase_balance(contract_address, 10);
    increase_balance(contract_address, 20);
    increase_balance(contract_address, 30);

    // Get the final balance
    let final_balance = get_balance(contract_address);

    // Assert that the final balance is the sum of all increases
    assert(final_balance == 60, 'Balance should be 60');
}
