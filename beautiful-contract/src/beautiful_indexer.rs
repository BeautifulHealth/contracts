#![no_std]

mod storage;

mod docs;
mod constants;
mod patient_info;
mod user_info;
mod user_management;

use constants::*;

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
