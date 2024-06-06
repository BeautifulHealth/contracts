#![no_std]

#[allow(unused_imports)]
use multiversx_sc::{
    imports::*,
    derive_imports::*,
};

/// This module defines the storage mappings and initialization for the Beautiful Health contract.
#[multiversx_sc::module]
pub trait StorageModule {

    /// Storage mapper for the access store address.
    /// Returns:
    /// - `SingleValueMapper<ManagedAddress>`: A mapper to store the address of the access store.
    #[storage_mapper("access_store")]
    fn access_store(&self) -> SingleValueMapper<ManagedAddress>;

    /// Storage mapper for the users address.
    /// Returns:
    /// - `SingleValueMapper<ManagedAddress>`: A mapper to store the address of the users contract.
    #[storage_mapper("users")]
    fn users(&self) -> SingleValueMapper<ManagedAddress>;
    
    /// Storage mapper for the EPR (Electronic Patient Record) index address.
    /// Returns:
    /// - `SingleValueMapper<ManagedAddress>`: A mapper to store the address of the EPR index.
    #[storage_mapper("epr_index")]
    fn epr_index(&self) -> SingleValueMapper<ManagedAddress>;
    
    /// Initializes the storage with the provided addresses for access store and users.
    /// Args:
    /// - `_access_store`: The address of the access store to be set.
    /// - `_users`: The address of the users contract to be set.
    #[endpoint]
    fn initStorage(&self, _access_store: ManagedAddress, _users: ManagedAddress) {
        self.access_store().set(_access_store);
        self.users().set(_users);
    }
}
