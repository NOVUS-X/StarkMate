// spectator_betting.cairo

%lang starknet

from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.cairo.common.uint256 import Uint256, uint256_add, uint256_mul, uint256_eq, uint256_div
from starkware.starknet.common.syscalls import get_caller_address, get_contract_address
from starkware.starknet.common.storage import Storage
from starkware.starknet.common.utils import assert_not_zero

@contract_interface
namespace IERC20:
    func transfer_from(sender: felt252, recipient: felt252, amount: Uint256) -> (res: felt):
    end

    func transfer(recipient: felt252, amount: Uint256) -> (res: felt):
    end
end

@contract_interface
namespace IMatchRegistry:
    func get_match_winner(match_id: felt252) -> (winner: felt252):
    end
end

struct Bet:
    winner: felt252,
    amount: Uint256,
    claimed: felt252
end

@storage_var
func bets(match_id: felt252, user: felt252) -> Bet:
end

@storage_var
func total_bets(match_id: felt252, winner: felt252) -> Uint256:
end

@storage_var
func total_pool(match_id: felt252) -> Uint256:
end

@storage_var
func token_address() -> felt252:
end

@storage_var
func match_registry_address() -> felt252:
end

@external
func init{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr}(token: felt252, registry: felt252):
    let existing_token = token_address.read();
    assert existing_token == 0;
    token_address.write(token);
    match_registry_address.write(registry);
    return ();
end

@external
func place_bet{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr}(match_id: felt252, winner: felt252, amount: Uint256):
    let (caller) = get_caller_address();

    let (existing_bet) = bets.read(match_id, caller);
    assert existing_bet.amount.low == 0;
    assert existing_bet.amount.high == 0;

    let (token) = token_address.read();
    let (contract_addr) = get_contract_address();
    let _ = IERC20.transfer_from(token).transfer_from(caller, contract_addr, amount);

    let new_bet = Bet(winner=winner, amount=amount, claimed=0);
    bets.write(match_id, caller, new_bet);

    let (current_total) = total_bets.read(match_id, winner);
    let (new_total) = uint256_add(current_total, amount);
    total_bets.write(match_id, winner, new_total);

    let (pool) = total_pool.read(match_id);
    let (new_pool) = uint256_add(pool, amount);
    total_pool.write(match_id, new_pool);

    return ();
end

@external
func distribute_winnings{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr}(match_id: felt252):
    let (caller) = get_caller_address();
    let (bet) = bets.read(match_id, caller);
    assert_not_zero(bet.amount.low + bet.amount.high);
    assert bet.claimed == 0;

    let (registry) = match_registry_address.read();
    let (winner) = IMatchRegistry.get_match_winner(registry).get_match_winner(match_id);
    assert_not_zero(winner);

    if bet.winner != winner {
        let updated = Bet(winner=bet.winner, amount=bet.amount, claimed=1);
        bets.write(match_id, caller, updated);
        return ();
    }

    let (total_for_winner) = total_bets.read(match_id, winner);
    let (total) = total_pool.read(match_id);
    let (payout) = uint256_div(uint256_mul(bet.amount, total), total_for_winner);

    let (token) = token_address.read();
    let _ = IERC20.transfer(token).transfer(caller, payout);

    let updated = Bet(winner=bet.winner, amount=bet.amount, claimed=1);
    bets.write(match_id, caller, updated);
    return ();
end
