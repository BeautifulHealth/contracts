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
use multiversx_sc::proxy_imports::heap::String;
use multiversx_sc::proxy_imports::heap::Vec;

use crate::{beautiful_utils};
use crate::beautiful_utils::ID;

pub type BHash = u64;
pub type Cid = String;
pub type Attr = beautiful_utils::Attribute;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DocumentGroup{
    pub attr: Vec<Attr>,
    pub cid_encrs: Vec<Cid>,
    pub user_groups: Vec<BHash>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DocGroupCreateParams <M: ManagedTypeApi> {
    pub group_id_hash: BHash,
    pub user_id_encr: BHash,
    pub attrs: Vec<Attr>,
    pub signer: ManagedAddress<M>,
    pub signature: BHash,
}

#[multiversx_sc::module]
pub trait DocGroupsModule {

    #[storage_mapper("docs_groups")]
    fn docs_groups(&self, user_id: ID) -> SingleValueMapper<DocumentGroup>;

}