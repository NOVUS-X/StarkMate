use core::option::OptionTrait;
use core::result::ResultTrait;
use core::traits::{Into, TryInto};
use match_result_storage::IMatchResultStorageDispatcher;
use match_staking::IMatchStakingDispatcher;
use starknet::ContractAddress;

// Constants
const ZERO_ADDRESS: ContractAddress = 0x0.try_into().unwrap();
const PLAYER1_WIN: felt252 = 1;
const PLAYER2_WIN: felt252 = 2;

#[starknet::interface]
pub trait IGameLifecycle<TContractState> {
    fn create_game(ref self: TContractState, player1: ContractAddress, stake: u256) -> felt252;
    fn join_game(ref self: TContractState, game_id: felt252) -> bool;
    fn submit_result(ref self: TContractState, game_id: felt252, winner: ContractAddress) -> bool;
    fn get_game_data(self: @TContractState, game_id: felt252) -> Game;
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub struct Game {
    pub player1: ContractAddress,
    pub player2: ContractAddress,
    pub stake: u256,
    pub status: GameStatus,
    pub created_at: u64,
    pub completed_at: u64,
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub enum GameStatus {
    Created,
    InProgress,
    Completed,
    Cancelled,
}

#[starknet::contract]
pub mod GameLifecycle {
    use openzeppelin_access::ownable::OwnableComponent;
    use starknet::storage::{Map, StorageMapReadAccess, StorageMapWriteAccess};
    use starknet::{ContractAddress, get_block_info, get_caller_address};
    use super::{Game, GameStatus, IGameLifecycle, PLAYER1_WIN, PLAYER2_WIN, ZERO_ADDRESS};

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);

    // External
    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;

    // Internal
    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        games: Map<felt252, Game>,
        next_game_id: felt252,
        match_staking: ContractAddress,
        match_result_storage: ContractAddress,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
    }

    #[derive(Drop, starknet::Event)]
    pub struct GameCreated {
        #[key]
        pub game_id: felt252,
        pub player1: ContractAddress,
        pub stake: u256,
    }

    #[derive(Drop, starknet::Event)]
    pub struct GameJoined {
        #[key]
        pub game_id: felt252,
        pub player2: ContractAddress,
    }

    #[derive(Drop, starknet::Event)]
    pub struct GameCompleted {
        #[key]
        pub game_id: felt252,
        pub winner: ContractAddress,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        GameCreated: GameCreated,
        GameJoined: GameJoined,
        GameCompleted: GameCompleted,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState,
        owner: ContractAddress,
        match_staking: ContractAddress,
        match_result_storage: ContractAddress,
    ) {
        // Validate addresses are non-zero
        assert(owner != ZERO_ADDRESS, 'Owner address cannot be zero');
        assert(match_staking != ZERO_ADDRESS, 'Match staking address cannot be zero');
        assert(match_result_storage != ZERO_ADDRESS, 'Match result storage address cannot be zero');

        self.ownable.initializer(owner);
        self.match_staking.write(match_staking);
        self.match_result_storage.write(match_result_storage);
        self.next_game_id.write(1);
    }

    #[abi(embed_v0)]
    impl GameLifecycleImpl of super::IGameLifecycle<ContractState> {
        fn create_game(ref self: ContractState, player1: ContractAddress, stake: u256) -> felt252 {
            // Only allow game creation by the player or contract owner
            let caller = get_caller_address();
            assert(
                caller == player1 || self.ownable.owner() == caller,
                'Caller must be player1 or owner',
            );

            // Generate new game ID
            let game_id = self.next_game_id.read();
            self.next_game_id.write(game_id + 1);

            // Create new game
            let timestamp = get_block_info().unbox().block_timestamp;
            let new_game = Game {
                player1,
                player2: ZERO_ADDRESS,
                stake,
                status: GameStatus::Created,
                created_at: timestamp,
                completed_at: 0,
            };

            self.games.write(game_id, new_game);

            // Create match in staking contract
            let match_staking = self.match_staking.read();
            let staking_dispatcher = IMatchStakingDispatcher { contract_address: match_staking };
            let create_success = staking_dispatcher.create_match(game_id, player1, stake);
            assert(create_success, 'Failed to create match in staking contract');

            // Emit event
            self.emit(GameCreated { game_id, player1, stake });

            game_id
        }

        fn join_game(ref self: ContractState, game_id: felt252) -> bool {
            // Only allow joining by the player2 or contract owner
            let caller = get_caller_address();
            assert(caller != ZERO_ADDRESS, 'Invalid player address');
            assert(caller != self.ownable.owner(), 'Owner cannot join as player2');

            // Get existing game
            let game = self.games.read(game_id);

            // Validate game state
            assert(game.status == GameStatus::Created, 'Game not available');
            assert(game.player2 == ZERO_ADDRESS, 'Game already joined');
            assert(game.player1 != caller, 'Cannot join your own game');

            // Join match in staking contract
            let match_staking = self.match_staking.read();
            let staking_dispatcher = IMatchStakingDispatcher { contract_address: match_staking };
            let join_success = staking_dispatcher.join_match(game_id, caller);
            assert(join_success, 'Failed to join match in staking contract');

            // Update game state
            let updated_game = Game {
                player1: game.player1,
                player2: caller,
                stake: game.stake,
                status: GameStatus::InProgress,
                created_at: game.created_at,
                completed_at: game.completed_at,
            };

            self.games.write(game_id, updated_game);

            // Emit event
            self.emit(GameJoined { game_id, player2: caller });

            true
        }

        fn submit_result(
            ref self: ContractState, game_id: felt252, winner: ContractAddress,
        ) -> bool {
            // Only allow result submission by the contract owner
            self.ownable.assert_only_owner();

            // Get existing game
            let game = self.games.read(game_id);

            // Validate game state
            assert(game.status == GameStatus::InProgress, 'Game not in progress');
            assert(winner == game.player1 || winner == game.player2, 'Invalid winner address');

            // Submit result to staking contract
            let match_staking = self.match_staking.read();
            let staking_dispatcher = IMatchStakingDispatcher { contract_address: match_staking };
            let claim_success = staking_dispatcher.claim_match(game_id, winner);
            assert(claim_success, 'Failed to claim match in staking contract');

            // Update game state
            let timestamp = get_block_info().unbox().block_timestamp;
            let updated_game = Game {
                player1: game.player1,
                player2: game.player2,
                stake: game.stake,
                status: GameStatus::Completed,
                created_at: game.created_at,
                completed_at: timestamp,
            };

            self.games.write(game_id, updated_game);

            // Store result in result storage
            let match_result_storage = self.match_result_storage.read();
            let result_storage_dispatcher = IMatchResultStorageDispatcher {
                contract_address: match_result_storage,
            };
            let result = if winner == game.player1 {
                PLAYER1_WIN
            } else {
                PLAYER2_WIN
            };
            result_storage_dispatcher.store_result(game_id, game.player1, game.player2, result);

            // Emit event
            self.emit(GameCompleted { game_id, winner });

            true
        }

        fn get_game_data(self: @ContractState, game_id: felt252) -> Game {
            self.games.read(game_id)
        }
    }
}
