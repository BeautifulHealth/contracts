#![no_std]

#[allow(unused_imports)]
use multiversx_sc::{
    imports::*,
    derive_imports::*,
};
use multiversx_sc::{
    api::{self, ManagedTypeApi},
    types::{ManagedAddress, ManagedBuffer, ManagedVec},
};
use multiversx_sc::proxy_imports::heap::String;
use multiversx_sc::proxy_imports::heap::Vec;

use crate::beautiful_utils;

pub type UserID = u64;
pub type EprID = u64;
pub type SubjectKey = u64;
pub type Cid = String;
pub type MetaArr<M> = Vec<beautiful_utils::DocMeta<M>>;

#[multiversx_sc::module]
pub trait DocsModule {
    #[storage_mapper("epr_users")]
    fn epr_users(&self, user_id: UserID) -> SingleValueMapper<EprID>;

    #[storage_mapper("epr_subject")]
    fn epr_subject(&self, subject_key: SubjectKey) -> SingleValueMapper<EprID>;

    #[storage_mapper("cids")]
    fn cids(&self, user_addr: ManagedAddress) -> SingleValueMapper<Cid>;

    // #[storage_mapper("epr_docs")]
    // fn epr_docs<M: multiversx_sc::api::ManagedTypeApi>(&self, epr_id: EprID) -> SingleValue<MetaArr<M>>;

    #[endpoint]
    fn set_epr_user(&self, id: UserID, epr_id: EprID) {
        self.epr_users(id).set(epr_id);
    }

    #[endpoint]
    fn get_epr_user(&self, id: UserID) -> EprID {
        self.epr_users(id).get()
    }

    #[endpoint]
    fn set_epr_subject(&self, subject_key: SubjectKey, id: EprID) {
        self.epr_subject(subject_key).set(id)
    }

    fn set_epr_doc_attribute(&self, id: EprID) {

        // let mut i = 0

        // while i < ehr
    }




    // function setEhrDocAttr(
    //     bytes32 ehrId,
    //     IDocs.Type docType,
    //     uint index,
    //     Attributes.Code attrCode,
    //     bytes memory value
    // )
    //     private
    // {
    //     for (uint i = 0; i < ehrDocs[ehrId][docType][index].attrs.length; i++) {
    //         if (ehrDocs[ehrId][docType][index].attrs[i].code == attrCode) {
    //             ehrDocs[ehrId][docType][index].attrs[i].value = value;
    //             return;
    //         }
    //     }
    // }




   

}


//     // ///
//     // function getEhrUser(bytes32 userIDHash) public view returns(bytes32) {
//     //     return ehrUsers[userIDHash];
//     // }
// }


