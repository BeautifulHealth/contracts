#![no_std]

pub mod beautiful_storage;
pub mod beautiful_utils;
pub mod beautiful_docs;
pub mod beautiful_doc_groups;
pub mod beautiful_users;
pub mod beautiful_events;
mod patient_info;
mod user_info;

#[allow(unused_imports)]
use multiversx_sc::{
    imports::*,
};
use multiversx_sc::{
    api::{self, ManagedTypeApi},
    types::{ManagedAddress, ManagedBuffer, ManagedVec},
};

/// This contract defines the Beautiful Health Indexer.
/// It includes initialization logic and storage mapping for the deployer address.
#[multiversx_sc::contract]
pub trait beautiful_indexer {

    /**
        Error codes: (To be implemented)
        Incorrect request
        Already deleted
        Not found
        Already exists
        Access denied
        Invalid signature
    */

    /// Storage mapper for the deployer address.
    /// Returns:
    /// - SingleValueMapper<ManagedAddress>: A mapper to store the address of the contract deployer.
    #[storage_mapper("deployer")]
    fn deployer(&self) -> SingleValueMapper<ManagedAddress>;


    /// Initializes the contract.
    /// Sets the deployer's address to the address of the caller.
    #[init]
    fn initialise(&self) {
        let deployer_addr = self.blockchain().get_caller();
        self.deployer().set(deployer_addr);

    }

}
