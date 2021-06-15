use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;

use lazy_static::lazy_static;
use log::*;
use mongodb::Collection;

use crate::address::ADDRESS_SERVICE;
use crate::db::{collection, MongodbCrudService};

use super::AddressBalance;

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
    #[tokio::main]
    pub async fn refresh_balance(&self) -> Result<String, reqwest::Error> {
        let api_key = env::var("API_KEY").expect("API_KEY env not set.");
        let addresses = ADDRESS_SERVICE.db_read_all_resources().await;
        info!("addresses {:?}", addresses.clone().unwrap());
        let mut url: String = "https://api-ropsten.etherscan.io/api?module=account&action=balance&address=0x560aba705254C7db2D46Cb978B7D31Ee02DC5039&tag=latest&apikey=".to_owned();
        url.push_str(&api_key);
        for mut addr in addresses.unwrap() {
            let resp = reqwest::get(&url)
                .await?
                .json::<HashMap<String, String>>()
                .await?;
            let balance = resp.get("result").unwrap();
            info!("Address {} has balance {}", addr.address, balance);

            let resp2 = self.db_create_resource(
                AddressBalance { id: None, address: addr.clone().address, balance: balance.clone() }).await;
            info!("saved balance {:?}", resp2.unwrap());
        }
        Ok(String::from("success"))
    }
}