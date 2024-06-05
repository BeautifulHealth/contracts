#[allow(unused_imports)]
use multiversx_sc::{
    imports::*,
    derive_imports::*,
};

use multiversx_sc::{
    api::ManagedTypeApi,
    types::{ManagedAddress, ManagedBuffer, ManagedVec},
};

pub type Version = u64;
pub type Timestamp = u64;
pub type ID = u64;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub enum Type {
    Ehr,
    EhrAccess,
    EhrStatus,
    Composition,
    Query,
    Template,
    Directory
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub enum Status {
  Active,
  Deleted,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DocMeta<M: ManagedTypeApi> {
    pub status: Status,
    pub id: Option<ID>,
    pub version: Option<Version>,
    pub timestamp: Option<Timestamp>,
    pub islast: bool,
    pub attr: ManagedVec<M, ManagedBuffer<M>>
}

impl<M> DocMeta<M>
where
    M: ManagedTypeApi,
{

}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub enum Code {
    Status,
    IDEncr,
    KeyEncr,
    DocUIDEncrypted,
    ContentEncr,
    DescriptionEncr,
    PasswordHash,
    Timestamp,
    NameEncr,
    GroupDoctorsIDHash,
    GroupAllDocsIDHash,
    DataIndexID
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Attribute {
    pub code: Code,
    pub value: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct AddEprDocParams<M: multiversx_sc::api::ManagedTypeApi> {
    pub doc_type: Type,
    pub id: u64,
    pub version: u8,
    pub timestamp: u64,
    pub signer: ManagedAddress<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DocumentGroup {
    pub id: u64,
}

// struct DocumentGroup {
//     mapping(bytes32 => bool)          CIDHashes;
//     Attributes.Attribute[]            attrs;
//     bytes[]                           CIDEncrs;   // CIDs encrypted with the group key 
//     bytes32[]                         userGroups; // userGroups that have access to this group
// }