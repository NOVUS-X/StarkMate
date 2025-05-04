use contract::ntf_trophy::{
    ISoulboundTrophyDispatcher, ISoulboundTrophyDispatcherTrait, TrophyDetails,
};
use openzeppelin_token::erc721::interface::{IERC721Dispatcher, IERC721DispatcherTrait};
use snforge_std::{ContractClassTrait, DeclareResultTrait, declare, start_cheat_caller_address};
use starknet::{ContractAddress, contract_address_const};

// Helper function to deploy the SoulboundTrophy contract
fn deploy_soulbound_contract(
    owner: ContractAddress, name: felt252, symbol: felt252,
) -> (ISoulboundTrophyDispatcher, IERC721Dispatcher) {
    let contract = declare("SoulboundTrophy").unwrap().contract_class();
    let calldata = array![owner.into(), name, symbol];
    let (contract_address, _) = contract.deploy(@calldata).unwrap();
    // Return dispatchers for both souldboundNft and standard ERC721 interfaces
    (ISoulboundTrophyDispatcher { contract_address }, IERC721Dispatcher { contract_address })
}

fn setup() -> (ISoulboundTrophyDispatcher, IERC721Dispatcher, ContractAddress) {
    let owner = contract_address_const::<1>();
    let name = 'Soulbound';
    let symbol = 'SBT';
    let (soulbound_contract, erc721_dispatcher) = deploy_soulbound_contract(owner, name, symbol);
    (soulbound_contract, erc721_dispatcher, owner)
}

#[test]
fn test_create_trophy() {
    let (soulbound_contract, _, owner) = setup();

    // Set caller to owner
    start_cheat_caller_address(soulbound_contract.contract_address, owner);

    let trophy_name = 'Trophy1';
    let trophy_desc = 'Desc1';
    let trophy_uri = 'Uri1';

    let trophy_id = soulbound_contract.create_trophy(trophy_name, trophy_desc, trophy_uri);
    let trophy_details = soulbound_contract.get_trophy_details(trophy_id);

    assert(trophy_id == 1, 'Trophy ID should be 1');
    assert(trophy_details.name == trophy_name, 'Name mismatch');
    assert(trophy_details.description == trophy_desc, 'Description mismatch');
    assert(trophy_details.uri == trophy_uri, 'URI mismatch');
}

#[test]
#[should_panic(expected_data: ('Caller is not the owner',))]
fn test_only_owner_can_create_trophy() {
    let (soulbound_contract, _, owner) = setup();

    let trophy_name = 'Trophy1';
    let trophy_desc = 'Desc1';
    let trophy_uri = 'Uri1';

    let trophy_id = soulbound_contract.create_trophy(trophy_name, trophy_desc, trophy_uri);
    let trophy_details = soulbound_contract.get_trophy_details(trophy_id);

    assert(trophy_id == 1, 'Trophy ID should be 1');
    assert(trophy_details.name == trophy_name, 'Name mismatch');
    assert(trophy_details.description == trophy_desc, 'Description mismatch');
    assert(trophy_details.uri == trophy_uri, 'URI mismatch');
}

#[test]
fn test_mint_trophy() {
    let player1 = contract_address_const::<1>();
    let (soulbound_contract, erc721_dispatcher, owner) = setup();

    // Set caller to owner
    start_cheat_caller_address(soulbound_contract.contract_address, owner);

    let trophy_name = 'Trophy1';
    let trophy_desc = 'Desc1';
    let trophy_uri = 'Uri1';

    let trophy_id = soulbound_contract.create_trophy(trophy_name, trophy_desc, trophy_uri);
    let trophy_details = soulbound_contract.get_trophy_details(trophy_id);
    soulbound_contract.mint_trophy(player1, trophy_id);

    assert(soulbound_contract.has_trophy(player1, trophy_id) == true, 'Player has no trophy ');
    assert(soulbound_contract.get_trophy_count(player1) == 1, 'No trophy found');

    // Testing on the erc721 contract
    assert(erc721_dispatcher.balance_of(player1) == 1, 'Player has no token minted');
    assert(erc721_dispatcher.owner_of(trophy_id) == player1, 'Player does not own this token');
}

#[test]
fn test_mint_multiple_trophies() {
    let player1 = contract_address_const::<2>();
    let (soulbound_contract, erc721_dispatcher, owner) = setup();
    start_cheat_caller_address(soulbound_contract.contract_address, owner);

    let trophy_id1 = soulbound_contract.create_trophy('Trophy1', 'Desc1', 'Uri1');
    let trophy_id2 = soulbound_contract.create_trophy('Trophy2', 'Desc2', 'Uri2');

    soulbound_contract.mint_trophy(player1, trophy_id1);
    soulbound_contract.mint_trophy(player1, trophy_id2);

    assert(soulbound_contract.get_trophy_count(player1) == 2, 'Trophy count should be 2');
    assert(soulbound_contract.has_trophy(player1, trophy_id1), 'Should have trophy 1');
    assert(soulbound_contract.has_trophy(player1, trophy_id2), 'Should have trophy 2');
    assert(erc721_dispatcher.balance_of(player1) == 2, 'ERC721 balance should be 2');
}

#[test]
#[should_panic(expected_data: ('Trophy already owned',))]
fn test_cannot_mint_same_trophy_twice() {
    let player1 = contract_address_const::<2>();
    let (soulbound_contract, _, owner) = setup();
    start_cheat_caller_address(soulbound_contract.contract_address, owner);

    let trophy_id = soulbound_contract.create_trophy('Trophy1', 'Desc1', 'Uri1');
    soulbound_contract.mint_trophy(player1, trophy_id);
    soulbound_contract.mint_trophy(player1, trophy_id);
}

#[test]
#[should_panic(expected_data: ('ERC721: Soulbound, transfers disabled',))]
fn test_transfer_disabled() {
    let player1 = contract_address_const::<2>();
    let player2 = contract_address_const::<3>();
    let (soulbound_contract, erc721_dispatcher, owner) = setup();
    start_cheat_caller_address(soulbound_contract.contract_address, owner);

    let trophy_id = soulbound_contract.create_trophy('Trophy1', 'Desc1', 'Uri1');
    soulbound_contract.mint_trophy(player1, trophy_id);

    let token_id = 1; // bascially trophy_id == token_id
    start_cheat_caller_address(erc721_dispatcher.contract_address, player1);
    erc721_dispatcher.transfer_from(player1, player2, token_id);
}
