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

pub type User <M> = beautiful_utils::User<M>;
pub type UserGroup <M> = beautiful_utils::UserGroup<M>;
pub type Role = beautiful_utils::Role;
pub type Attribute = beautiful_utils::Attribute;
// const empty_buff: ManagedBuffer<M> = ManagedBuffer::is;

#[multiversx_sc::module]
pub trait UsersModule {

    #[storage_mapper("users_store")]
    fn users_store<M: multiversx_sc::api::ManagedTypeApi>(&self, user: &ManagedAddress) -> SingleValueMapper<User<M>>;

    #[storage_mapper("user_groups")]
    fn users_groups<M: multiversx_sc::api::ManagedTypeApi>(&self, group_hash: ManagedBuffer) -> SingleValueMapper<UserGroup<M>>;

    #[storage_mapper("user_codes")]
    fn user_codes(&self, user_id: u64) -> SingleValueMapper<ManagedAddress>;

    #[endpoint]
    fn user_new(&self, _addr: ManagedAddress, _id_hash: ManagedBuffer, _role: Role, _attr: Vec<Attribute>) {

        let temp_hash: ManagedBuffer= self.users_store(&_addr).get().ID_hash;
        let hash_stat: bool = temp_hash.is_empty();
        require!(
            hash_stat, "already exists"
        );

        let _user = User { 
            ID_hash: _id_hash, 
            role: _role 
        };
        self.users_store(&_addr).set(_user);

    }

}