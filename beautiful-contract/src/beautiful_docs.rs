#![no_std]

#[allow(unused_imports)]
use multiversx_sc::{
    imports::*,
};

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait beautiful_docs {

    #[storage_mapper("epr_user")]
    fn epr_users(&self) -> MapMapper<u32, u32>;

    #[storage_mapper("epr_subject")]
    fn epr_subject(&self) -> MapMapper<u32, u32>;
    
    #[storage_mapper("cids")]
    fn cids(&self) -> MapMapper<u32, bool>;


    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}


    #[endpoint]
    fn set_epr_user(&self) {

    }

    
}
