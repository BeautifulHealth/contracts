#![no_std]

#[allow(unused_imports)]
use multiversx_sc::{
    imports::*,
};

#[multiversx_sc::module]
pub trait StorageModule {

    #[storage_mapper("access_store")]
    fn access_store(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("users")]
    fn users(&self) -> SingleValueMapper<ManagedAddress>;
    
    #[storage_mapper("epr_index")]
    fn epr_index(&self) -> SingleValueMapper<ManagedAddress>;


    // #[init]
    // fn init(&self, _access_store: ManagedAddress, _users: ManagedAddress, _epr_index: ManagedAddress) {
    //     self.access_store().set(_access_store);
    //     self.users().set(_users);
    //     self.epr_index().set(_epr_index);
    // }

    
}