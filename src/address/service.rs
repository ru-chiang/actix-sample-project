use std::collections::HashMap;
use std::env;

use lazy_static::lazy_static;
use log::*;
use mongodb::Collection;

use crate::db::{collection, MongodbCrudService};

use super::Address;
use std::borrow::Borrow;
use crate::utils::common::ApplicationError;

lazy_static! {
    pub static ref ADDRESS_SERVICE: AddressService = AddressService::new();
}

pub struct AddressService {}

impl MongodbCrudService<Address> for AddressService {
    fn new() -> AddressService {
        AddressService {}
    }

    fn table(&self) -> Collection {
        collection(Address::COLLECTION_NAME)
    }
}
