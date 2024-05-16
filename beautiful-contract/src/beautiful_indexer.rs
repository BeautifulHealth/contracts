#![no_std]

pub mod beautiful_docs;
mod helpers;
mod constants;
mod storage;
mod patient_info;

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

    
}
