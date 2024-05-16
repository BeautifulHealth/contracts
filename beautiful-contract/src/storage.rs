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
    
}
