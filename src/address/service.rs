use lazy_static::lazy_static;
use log::*;
use mongodb::Collection;

use crate::db::{collection, MongodbCrudService};
use crate::utils::common::ApplicationError;

use super::Address;

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

impl AddressService {
    pub async fn add_new_address(&self, addr: Address) -> Result<Address, ApplicationError> {
        let coll = self.table();
        let res = coll.find_one(
            Some(doc! {"address" => addr.address.to_owned()}), None,
        ).unwrap();
        let address = &addr.address[..];
        let mut s = "address".to_string();
        s.push_str(address);
        s.push_str(" already exists");
        match res {
            Some(_) => {
                error!("add_new_address::{}", s);
                Err(ApplicationError::ValidationError(String::from("address")))
            }
            None => {
                self.db_create_resource(addr.clone()).await
            }
        }
    }
}
