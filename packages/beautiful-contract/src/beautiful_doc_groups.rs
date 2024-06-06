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

/// Type alias for a hash value.
pub type BHash = u64;

/// Type alias for a content identifier.
pub type Cid = String;

/// Type alias for an attribute defined in beautiful_utils.
pub type Attr = beautiful_utils::Attribute;

/// Represents a group of documents with attributes, encrypted content identifiers, and user groups.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DocumentGroup {
    /// List of attributes associated with the document group.
    pub attr: Vec<Attr>,
    /// List of encrypted content identifiers.
    pub cid_encrs: Vec<Cid>,
    /// List of user group hashes.
    pub user_groups: Vec<BHash>,
}

/// Parameters required to create a document group.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DocGroupCreateParams<M: ManagedTypeApi> {
    /// Hash of the document group ID.
    pub group_id_hash: BHash,
    /// Encrypted user ID hash.
    pub user_id_encr: BHash,
    /// List of attributes for the document group.
    pub attrs: Vec<Attr>,
    /// Address of the signer.
    pub signer: ManagedAddress<M>,
    /// Signature hash for verification.
    pub signature: BHash,
}

/// Trait defining the document groups module for managing document groups.
#[multiversx_sc::module]
pub trait DocGroupsModule {

    /// Storage mapper for document groups associated with a user ID.
    /// Args:
    /// - `user_id`: The identifier of the user.
    /// Returns:
    /// - `SingleValueMapper<DocumentGroup>`: A mapper to store and retrieve the document group associated with the user ID.
    #[storage_mapper("docs_groups")]
    fn docs_groups(&self, user_id: ID) -> SingleValueMapper<DocumentGroup>;

}