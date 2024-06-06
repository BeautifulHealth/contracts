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

#[multiversx_sc::module]
pub trait EventsModule {

    #[event("msgSenderUpdated")]
    fn msg_sender_updated_event(
        &self,
        #[indexed] deployer: ManagedAddress,
    );

}