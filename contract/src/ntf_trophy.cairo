use starknet::ContractAddress;

#[starknet::interface]
pub trait ISoulboundTrophy<TContractState> {
    fn create_trophy(ref self: TContractState, name: felt252, description: felt252, metadata_uri: felt252) -> u256;
    fn mint_trophy(ref self: TContractState, player: ContractAddress, trophy_id: u256);
    fn has_trophy(self: @TContractState, player: ContractAddress, trophy_id: u256) -> bool;
    fn get_trophy_count(self: @TContractState, player: ContractAddress) -> u256;
    fn get_trophy_details(self: @TContractState, trophy_id: u256) -> TrophyDetails;
}

#[starknet::interface]
pub trait IERC721<TContractState> {
    fn balance_of(self: @TContractState, owner: ContractAddress) -> u256;
    fn owner_of(self: @TContractState, token_id: u256) -> ContractAddress;
    fn safe_transfer_from(ref self: TContractState, from: ContractAddress, to: ContractAddress, token_id: u256, data: Span<felt252>);
    fn transfer_from(ref self: TContractState, from: ContractAddress, to: ContractAddress, token_id: u256);
    fn approve(ref self: TContractState, approved: ContractAddress, token_id: u256);
    fn set_approval_for_all(ref self: TContractState, operator: ContractAddress, approved: bool);
    fn get_approved(self: @TContractState, token_id: u256) -> ContractAddress;
    fn is_approved_for_all(self: @TContractState, owner: ContractAddress, operator: ContractAddress) -> bool;
}

#[starknet::interface]
pub trait IERC721Metadata<TContractState> {
    fn name(self: @TContractState) -> ByteArray;
    fn symbol(self: @TContractState) -> ByteArray;
    fn token_uri(self: @TContractState, token_id: u256) -> ByteArray;
}

#[derive(Drop, Copy, starknet::Store, Serde)]
pub struct Trophy {
    pub name: felt252,
    pub description: felt252,
    pub uri: felt252,
}

#[derive(Drop, starknet::Store, Serde)]
pub struct TrophyDetails {
    pub name: felt252,
    pub description: felt252,
    pub uri: felt252,
}

#[starknet::contract]
pub mod SoulboundTrophy {
    use super::ContractAddress;
    use starknet::contract_address_const;
    use starknet::storage::{Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess, StoragePointerWriteAccess};
    use openzeppelin_access::ownable::OwnableComponent;
    use super::{Trophy, TrophyDetails};

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);

    #[abi(embed_v0)]
    impl OwnableMixinImpl = OwnableComponent::OwnableMixinImpl<ContractState>;

    impl OwnableInternalImpl = OwnableComponent::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        // Trophy data by trophy_id
        trophies: Map<u256, Trophy>, 
        // Tracks if player owns trophy_id
        user_trophies: Map<(ContractAddress, u256), bool>, 
        // Total trophies per player
        user_trophy_count: Map<ContractAddress, u256>, 
        // Next trophy type ID
        next_trophy_id: u256,
        // Number of tokens per owner
        balances: Map<ContractAddress, u256>, 
        // Token_id to owner
        owners: Map<u256, ContractAddress>, 
        // Maps token_id to trophy_id
        token_to_trophy: Map<u256, u256>, 
        // Next token ID for minting
        next_token_id: u256, 
        name: felt252,
        symbol: felt252,
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
    }

    #[derive(Drop, starknet::Event)]
    pub struct TrophyMinted {
        #[key]
        pub player: ContractAddress,
        #[key]
        pub trophy_id: u256,
        pub trophy_name: felt252,
    }

    #[derive(Drop, starknet::Event)]
    pub struct TrophyMetadataUpdated {
        #[key]
        pub trophy_id: u256,
        pub metadata_uri: felt252,
    }

    #[derive(Drop, starknet::Event)]
    pub struct Transfer {
        pub from: ContractAddress,
        pub to: ContractAddress,
        pub token_id: u256,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        OwnableEvent: OwnableComponent::Event,
        TrophyMinted: TrophyMinted,
        TrophyMetadataUpdated: TrophyMetadataUpdated,
        Transfer: Transfer,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress, _name: felt252, _symbol: felt252) {
        self.ownable.initializer(owner);
        self.name.write(_name); 
        self.symbol.write(_symbol);
        self.next_trophy_id.write(1);
        self.next_token_id.write(1);
    }

    #[abi(embed_v0)]
    impl SoulboundTrophyImpl of super::ISoulboundTrophy<ContractState> {
        fn create_trophy(
            ref self: ContractState,
            name: felt252,
            description: felt252,
            metadata_uri: felt252
        ) -> u256 {
            self.ownable.assert_only_owner();
            
            let trophy_id = self.next_trophy_id.read();
            let trophy = Trophy {
                name,
                description,
                uri: metadata_uri,
            };

            self.trophies.write(trophy_id, trophy);
            self.next_trophy_id.write(trophy_id + 1);
            
            self.emit(TrophyMetadataUpdated { trophy_id, metadata_uri });
            
            trophy_id
        }

        fn mint_trophy(ref self: ContractState, player: ContractAddress, trophy_id: u256) {
            assert(player != contract_address_const::<0>(), 'Invalid address');
            
            let already_has_trophy = self.user_trophies.read((player, trophy_id));
            assert(!already_has_trophy, 'Trophy already owned');
            
            let trophy = self._get_trophy(trophy_id);
            
            let token_id = self.next_token_id.read();
            self.next_token_id.write(token_id + 1);
            
            self.balances.write(player, self.balances.read(player) + 1);
            self.owners.write(token_id, player);
            self.token_to_trophy.write(token_id, trophy_id);
            
            self.user_trophies.write((player, trophy_id), true);
            let user_count = self.user_trophy_count.read(player);
            self.user_trophy_count.write(player, user_count + 1);
            
            self.emit(TrophyMinted { player, trophy_id, trophy_name: trophy.name });
            self.emit(Transfer { from: contract_address_const::<0>(), to: player, token_id });
        }

        fn has_trophy(self: @ContractState, player: ContractAddress, trophy_id: u256) -> bool {
            self.user_trophies.read((player, trophy_id))
        }

        fn get_trophy_count(self: @ContractState, player: ContractAddress) -> u256 {
            self.user_trophy_count.read(player)
        }

        fn get_trophy_details(self: @ContractState, trophy_id: u256) -> TrophyDetails {
            let trophy = self._get_trophy(trophy_id);
            TrophyDetails {
                name: trophy.name,
                description: trophy.description,
                uri: trophy.uri,
            }
        }
    }

    #[abi(embed_v0)]
    impl ERC721Impl of super::IERC721<ContractState> {
        fn balance_of(self: @ContractState, owner: ContractAddress) -> u256 {
            assert(owner != contract_address_const::<0>(), 'ERC721: Invalid address');
            self.balances.read(owner)
        }

        fn owner_of(self: @ContractState, token_id: u256) -> ContractAddress {
            let owner = self.owners.read(token_id);
            assert(owner != contract_address_const::<0>(), 'Token does not exist');
            owner
        }

        fn safe_transfer_from(
            ref self: ContractState,
            from: ContractAddress,
            to: ContractAddress,
            token_id: u256,
            data: Span<felt252>,
        ) {
            panic!("ERC721: Soulbound, transfers disabled")
        }

        fn transfer_from(
            ref self: ContractState,
            from: ContractAddress,
            to: ContractAddress,
            token_id: u256,
        ) {
            panic!("ERC721: Soulbound, transfers disabled")
        }

        fn approve(ref self: ContractState, approved: ContractAddress, token_id: u256) {
            panic!("ERC721: Soulbound, approvals disabled")
        }

        fn set_approval_for_all(ref self: ContractState, operator: ContractAddress, approved: bool) {
            panic!("ERC721: Soulbound, approvals disabled")
        }

        fn get_approved(self: @ContractState, token_id: u256) -> ContractAddress {
            contract_address_const::<0>()
        }

        fn is_approved_for_all(
            self: @ContractState,
            owner: ContractAddress,
            operator: ContractAddress,
        ) -> bool {
            false
        }
    }

    #[abi(embed_v0)]
    impl ERC721MetadataImpl of super::IERC721Metadata<ContractState> {
        fn name(self: @ContractState) -> ByteArray {
            format!("{}", self.name.read())
        }

        fn symbol(self: @ContractState) -> ByteArray {
            format!("{}", self.symbol.read())
        }

        fn token_uri(self: @ContractState, token_id: u256) -> ByteArray {
            let owner = self.owners.read(token_id);
            assert(owner != contract_address_const::<0>(), 'Token does not exist');
            let trophy_id = self.token_to_trophy.read(token_id);
            let trophy = self._get_trophy(trophy_id);
            format!("{}", trophy.uri)
        }
    }

    #[generate_trait]
    impl InternalImpl of InternalTrait {
        fn _get_trophy(self: @ContractState, trophy_id: u256) -> Trophy {
            assert(trophy_id > 0 && trophy_id < self.next_trophy_id.read(), 'Trophy does not exist');
            self.trophies.read(trophy_id)
        }
    }
}