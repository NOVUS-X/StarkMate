%lang starknet

from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.cairo.common.uint256 import Uint256, uint256_add
from starkware.starknet.testing.starknet import Starknet
from starkware.starknet.testing.contract import StarknetContract
from starkware.starknet.common.syscalls import get_caller_address

from spectator_betting import Bet

@view
func dummy_match_winner(match_id: felt252) -> (winner: felt252):
    return (winner=11);
end

@external
func dummy_transfer_from(sender: felt252, recipient: felt252, amount: Uint256) -> (res: felt):
    return (res=1);
end

@external
func dummy_transfer(recipient: felt252, amount: Uint256) -> (res: felt):
    return (res=1);
end

@external
func test_place_and_distribute{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr}() -> ():
    let token = 0x9999;
    let match_registry = 0x7777;
    let match_id = 1;
    let user = 0x1234;
    let winner = 11;
    let amount = Uint256(low=100, high=0);

    let betting = deploy_contract('spectator_betting', constructor_calldata=[]);
    betting.init(token, match_registry);

    set_caller_address(user);
    betting.place_bet(match_id, winner, amount);

    let (stored_bet) = betting.bets(match_id, user);
    assert stored_bet.winner == winner;
    assert stored_bet.amount.low == amount.low;

    # simulate match result
    let (winner_fetched) = dummy_match_winner(match_id);
    assert winner_fetched == winner;

    betting.distribute_winnings(match_id);
    let (updated_bet) = betting.bets(match_id, user);
    assert updated_bet.claimed == 1;

    return ();
end
