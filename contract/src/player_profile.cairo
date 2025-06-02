use starknet::ContractAddress;

#[starknet::interface]
pub trait IPlayerProfile<TContractState> {
    fn create_profile(ref self: TContractState, address: ContractAddress, username: felt252);
    fn update_stats(ref self: TContractState, address: ContractAddress, win: u32, loss: u32);
    fn get_profile(self: @TContractState, address: ContractAddress) -> PlayerProfile;
    fn get_username(self: @TContractState, address: ContractAddress) -> felt252;
    fn get_stats(self: @TContractState, address: ContractAddress) -> PlayerStats;
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub struct PlayerStats {
    pub games_played: u32,
    pub wins: u32,
    pub losses: u32,
    pub ranking: u32,
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub struct PlayerProfile {
    pub address: ContractAddress,
    pub username: felt252,
    pub stats: PlayerStats,
    pub is_active: bool,
}

#[starknet::contract]
pub mod PlayerProfile {
    use starknet::storage::{Map, StorageMapReadAccess, StorageMapWriteAccess};
    use starknet::{ContractAddress, get_caller_address};
    use super::{PlayerProfile, PlayerStats};

    #[storage]
    struct Storage {
        // Mapping from address to profile
        profiles: Map<ContractAddress, PlayerProfile>,
        // Mapping from username to address for uniqueness check
        username_to_address: Map<felt252, ContractAddress>,
    }

    #[derive(Drop, starknet::Event)]
    pub struct ProfileCreated {
        #[key]
        pub address: ContractAddress,
        pub username: felt252,
    }

    #[derive(Drop, starknet::Event)]
    pub struct StatsUpdated {
        #[key]
        pub address: ContractAddress,
        pub games_played: u32,
        pub wins: u32,
        pub losses: u32,
        pub ranking: u32,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        ProfileCreated: ProfileCreated,
        StatsUpdated: StatsUpdated,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {}

    #[abi(embed_v0)]
    impl PlayerProfileImpl of super::IPlayerProfile<ContractState> {
        fn create_profile(ref self: ContractState, address: ContractAddress, username: felt252) {
            // Ensure caller is the address being registered
            let caller = get_caller_address();
            assert(caller == address, 'Only address owner can create profile');

            // Check if profile already exists
            let existing_profile = self.profiles.read(address);
            assert(!existing_profile.is_active, 'Profile already exists');

            // Check if username is already taken
            let existing_address = self.username_to_address.read(username);
            assert(existing_address == 0.try_into().unwrap(), 'Username already taken');

            // Create new profile with initial stats
            let initial_stats = PlayerStats {
                games_played: 0,
                wins: 0,
                losses: 0,
                ranking: 0,
            };

            let new_profile = PlayerProfile {
                address,
                username,
                stats: initial_stats,
                is_active: true,
            };

            // Store profile and username mapping
            self.profiles.write(address, new_profile);
            self.username_to_address.write(username, address);

            // Emit event
            self.emit(ProfileCreated { address, username });
        }

        fn update_stats(ref self: ContractState, address: ContractAddress, win: u32, loss: u32) {
            // Get existing profile
            let mut profile = self.profiles.read(address);
            assert(profile.is_active, 'Profile does not exist');

            // Update stats
            let mut stats = profile.stats;
            stats.games_played += win + loss;
            stats.wins += win;
            stats.losses += loss;

            // Calculate new ranking (simple implementation - can be enhanced)
            if stats.games_played > 0 {
                stats.ranking = (stats.wins * 100) / (stats.games_played);
            }

            // Update profile
            profile.stats = stats;
            self.profiles.write(address, profile);

            // Emit event
            self.emit(StatsUpdated {
                address,
                games_played: stats.games_played,
                wins: stats.wins,
                losses: stats.losses,
                ranking: stats.ranking,
            });
        }

        fn get_profile(self: @ContractState, address: ContractAddress) -> PlayerProfile {
            let profile = self.profiles.read(address);
            assert(profile.is_active, 'Profile does not exist');
            profile
        }

        fn get_username(self: @ContractState, address: ContractAddress) -> felt252 {
            let profile = self.profiles.read(address);
            assert(profile.is_active, 'Profile does not exist');
            profile.username
        }

        fn get_stats(self: @ContractState, address: ContractAddress) -> PlayerStats {
            let profile = self.profiles.read(address);
            assert(profile.is_active, 'Profile does not exist');
            profile.stats
        }
    }
} 