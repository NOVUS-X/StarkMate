use starknet::ContractAddress;

#[starknet::interface]
pub trait IMatchResultStorage<TContractState> {
    fn store_result(
        ref self: TContractState,
        match_id: felt252,
        player_white: ContractAddress,
        player_black: ContractAddress,
        result: felt252,
    );
    fn get_result(self: @TContractState, match_id: felt252) -> MatchResult;
    fn get_oracle_address(self: @TContractState) -> ContractAddress;
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub struct MatchResult {
    pub match_id: felt252,
    pub player_white: ContractAddress,
    pub player_black: ContractAddress,
    pub result: felt252,
    pub timestamp: u64,
}

#[starknet::contract]
pub mod MatchResultStorage {
    use core::array::Array;
    use core::box::BoxTrait;
    use core::felt252;
    use core::integer::u64;
    use core::traits::Into;
    use starknet::storage::*;
    use starknet::{ContractAddress, get_block_info, get_caller_address};
    use super::MatchResult;

    #[storage]
    pub struct Storage {
        // The address of the oracle allowed to store results
        oracle_address: ContractAddress,
        // Mapping from match_id to MatchResult struct
        results: Map<felt252, MatchResult>,
    }

    #[derive(Drop, starknet::Event)]
    pub struct MatchResultStored {
        #[key]
        pub match_id: felt252,
        pub player_white: ContractAddress,
        pub player_black: ContractAddress,
        pub result: felt252,
        pub timestamp: u64,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        MatchResultStored: MatchResultStored,
    }

    #[constructor]
    fn constructor(ref self: ContractState, oracle_address: ContractAddress) {
        self.oracle_address.write(oracle_address);
    }

    #[abi(embed_v0)]
    pub impl MatchResultStorageImpl of super::IMatchResultStorage<ContractState> {
        // Store the final match result
        fn store_result(
            ref self: ContractState,
            match_id: felt252,
            player_white: ContractAddress,
            player_black: ContractAddress,
            result: felt252,
        ) {
            // Ensure only the oracle can call this function
            let caller = get_caller_address();
            let oracle = self.oracle_address.read();
            assert(caller == oracle, 'Only oracle can store result');

            // Get the current block timestamp
            let timestamp = get_block_info().unbox().block_timestamp;

            // Create and store the MatchResult struct
            let match_result = MatchResult {
                match_id, player_white, player_black, result, timestamp,
            };
            self.results.write(match_id, match_result);

            // Emit event for indexers
            self
                .emit(
                    MatchResultStored { match_id, player_white, player_black, result, timestamp },
                );
        }

        // Retrieve a match result by its ID
        fn get_result(self: @ContractState, match_id: felt252) -> MatchResult {
            self.results.read(match_id)
        }

        // Get the oracle address
        fn get_oracle_address(self: @ContractState) -> ContractAddress {
            self.oracle_address.read()
        }
    }
}
