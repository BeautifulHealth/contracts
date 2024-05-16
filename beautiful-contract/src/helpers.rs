use multiversx_sc::imports::*;

use crate::{
    constants::*,
    patient_info::{PatientInfo, AccessLog},
    user_info::{UserInfo},
    storage,
};

#[multiversx_sc::module]
pub trait HelpersModule: storage::StorageModule {

}