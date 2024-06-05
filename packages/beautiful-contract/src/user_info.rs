use multiversx_sc::{derive_imports::*, imports::*};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Copy)]
pub enum UserRole {
    Administrator,
    Moderator,
    Doctor,
    Patient,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct UserInfo<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub role: UserRole,
    pub date_registered: u64,
}

impl<M> UserInfo<M>
where 
    M: ManagedTypeApi, 
{
    pub fn has_role(&self, role: UserRole) -> bool {
        self.role == role
    }
}