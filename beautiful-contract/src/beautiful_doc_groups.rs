#![no_std]

#[allow(unused_imports)]
use multiversx_sc::{
    imports::*,
    derive_imports::*,
};
use multiversx_sc::{
    api::ManagedTypeApi,
    types::{ManagedAddress, ManagedBuffer, ManagedVec},
};

use crate::beautiful_doc_groups;

pub type UserID = u64;
// pub type DocumentGroup = beautiful_doc_groups::


#[multiversx_sc::module]
pub trait DocGroupsModule {

    #[storage_mapper("docs_groups")]
    fn docs_groups(&self, user_id: UserID) -> SingleValueMapper<UserID>;




    // mapping (bytes32 => DocumentGroup) docGroups;  // idHash => DocumentGroup

}