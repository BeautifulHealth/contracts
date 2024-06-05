// #![no_std]

// #[allow(unused_imports)]
// use multiversx_sc::{
//     imports::*,
//     derive_imports::*,
// };


// #[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
// enum Code {
//     Status,
//     IDEncr,
//     KeyEncr,
//     DocUIDEncrypted,
//     ContentEncr,
//     DescriptionEncr,
//     PasswordHash,
//     Timestamp,
//     NameEncr,
//     GroupDoctorsIDHash,
//     GroupAllDocsIDHash,
//     DataIndexID
// }

// #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
// pub struct Attribute {
//     pub code: Code,
//     pub value: u64,
// }

// #[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
// enum Status {
//   Active,
//   Deleted,
// }

// #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
// pub struct DocMeta {
//     pub status: Status,
//     pub id: u64,
//     pub version: u8,
//     pub timestamp: u64,
//     // pub attr: ManagedVec<M, Attribute<M>>,
// }

// #[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
// enum Type {
//     Ehr,
//     EhrAccess,
//     EhrStatus,
//     Composition,
//     Query,
//     Template,
//     Directory
// }

// #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
// pub struct AddEprDocParams<M: multiversx_sc::api::ManagedTypeApi> {
//     pub docType: Type,
//     pub id: u64,
//     pub version: u8,
//     pub timestamp: u64,
//     pub signer: ManagedAddress<M>,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// #[multiversx_sc::module]
// pub trait DocsModule {


//     #[storage_mapper("epr_users")]
//     fn epr_users(&self) -> MapMapper<u64, u64>;

//     #[storage_mapper("epr_subject")]
//     fn epr_subject(&self) -> MapMapper<u64, u64>;

//     #[storage_mapper("cids")]
//     fn cids(&self) -> MapMapper<u64, bool>;

//     #[endpoint]
//     fn setEprUser(&self, Id: u64, EprId: u64) -> Result<(), Err> {
//         self.epr_users().insert(Id, EprId);

//         Ok(())
//     } 



//     // function setEhrUser(
//     //     bytes32 IDHash,
//     //     bytes32 ehrId,
//     //     address signer,
//     //     uint deadline,
//     //     bytes calldata signature
//     // )
//     //     external onlyAllowed(msg.sender)
//     // {
//     //     signCheck(signer, deadline, signature);
//     //     require(ehrUsers[IDHash] == bytes32(0), "AEX");
//     //     ehrUsers[IDHash] = ehrId;
//     // }

//     // ///
//     // function getEhrUser(bytes32 userIDHash) public view returns(bytes32) {
//     //     return ehrUsers[userIDHash];
//     // }
// }


