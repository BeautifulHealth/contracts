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

#[multiversx_sc::module]
pub trait UsersModule {

    #[storage_mapper("users_store")]
    fn users_store<M: multiversx_sc::api::ManagedTypeApi>(&self, user: ManagedAddress) -> SingleValueMapper<User<M>>;

    #[storage_mapper("user_groups")]
    fn users_groups<M: multiversx_sc::api::ManagedTypeApi>(&self, group_hash: ManagedBuffer) -> SingleValueMapper<UserGroup<M>>;

    #[storage_mapper()]
}