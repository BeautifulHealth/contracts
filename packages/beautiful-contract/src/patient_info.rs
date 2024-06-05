use multiversx_sc::{derive_imports::*, imports::*};

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct PatientInfo<M: ManagedTypeApi> {
    pub patient_address: ManagedAddress<M>,
    pub patient_id: u64,
    pub health_data: ManagedBuffer<M>,
    pub access_permissions: ManagedVec<M, ManagedAddress<M>>,
    pub date_created: u64,
    pub last_updated: u64,
}

impl<M> PatientInfo<M>
where 
    M: ManagedTypeApi, 
{

    pub fn grant_access(&mut self, address: ManagedAddress<M>) {
        self.access_permissions.push(address);
    }

    pub fn revoke_access(&mut self, address: ManagedAddress<M>) -> Result<(), &'static str> {
        let index = self.access_permissions.iter().position(|x| *x == address).ok_or("Address not found")?;
        self.access_permissions.remove(index);

        Ok(())
    }

    pub fn update_record(&mut self, caller: ManagedAddress<M>, data: ManagedBuffer<M>) -> Result<(), ()> {
        if (self.access_permissions.iter().position(|x| *x == caller)).is_some() {
            self.health_data = data;
            Ok(())
        } else {
            Err(())
        }
    }
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct AccessLog<M: ManagedTypeApi> {
    pub access_time: u64,
    pub accessed_by: ManagedAddress<M>,
    pub record_id: ManagedBuffer<M>
}