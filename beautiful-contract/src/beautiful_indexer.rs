#![no_std]


pub mod beautiful_docs;

#[allow(unused_imports)]
use multiversx_sc::{
    imports::*,
};

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait beautiful_indexer {


    #[init]
    fn init(&self) {}

    
}
