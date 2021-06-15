use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;

use lazy_static::lazy_static;
use log::*;
use mongodb::Collection;

use crate::address::ADDRESS_SERVICE;
use crate::db::{collection, MongodbCrudService};
use crate::utils::common::BusinessError;
use crate::utils::request::get_balance_by_address;
use super::AddressBalance;

use serde_json::Value;
use serde_json::value::Value::Null;

lazy_static! {
    pub static ref ADDRESS_BALANCE_SERVICE: AddressBalanceService = AddressBalanceService::new();
}

pub struct AddressBalanceService {}

impl MongodbCrudService<AddressBalance> for AddressBalanceService {
    fn new() -> AddressBalanceService {
        AddressBalanceService {}
    }

    fn table(&self) -> Collection {
        collection(AddressBalance::COLLECTION_NAME)
    }
}

impl AddressBalanceService {
    pub async fn refresh_balance(&self) -> Result<Value, BusinessError> {
        let addresses = ADDRESS_SERVICE.db_read_all_resources().await?;
        info!("addresses {:?}", addresses);

        for mut addr in addresses {
            let balance = get_balance_by_address(&addr.address).await?;
            info!("Address {} has balance {}", addr.address, balance);

            let resp2 = self.db_create_resource(
                AddressBalance {
                    id: None,
                    address: addr.address.to_owned(),
                    balance: balance.to_owned(),
                }).await;
            info!("saved balance {:?}", resp2.unwrap());
        }
        Ok(Null)
    }
}