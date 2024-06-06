#![no_std]

pub mod beautiful_storage;
pub mod beautiful_utils;
pub mod beautiful_docs;
pub mod beautiful_doc_groups;
pub mod beautiful_users;
mod patient_info;
mod user_info;


#[allow(unused_imports)]
use multiversx_sc::{
    imports::*,
};

#[multiversx_sc::contract]
pub trait beautiful_indexer {


    #[init]
    fn initialise(&self) {
    }

    #[endpoint]
    fn register_user(&self) {} 

    
}
