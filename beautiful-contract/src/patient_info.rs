use multiversx_sc::{derive_imports::*, imports::*};

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct PatientInfo<M: ManagedTypeApi> {
    pub patient_address: ManagedAddress<M>,
    pub health_data: ManagedBuffer<M>,
    pub access_permissions: ManagedVec<M, ManagedAddress<M>>,
    pub date_created: u64,
    pub last_updated: u64,
}

impl<M> PatientInfo<M>
where 
    M: ManagedTypeApi, 
{

    // pub fn grant_access
}