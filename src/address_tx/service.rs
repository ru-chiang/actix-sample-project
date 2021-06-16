use lazy_static::lazy_static;
use log::*;
use mongodb::Collection;

use crate::address::ADDRESS_SERVICE;
use crate::db::{collection, MongodbCrudService};
use crate::utils::common::ApplicationError;
use crate::utils::request::get_tx_by_address;

use super::AddressTx;

lazy_static! {
    pub static ref ADDRESS_TX_SERVICE: AddressTxService = AddressTxService::new();
}

pub struct AddressTxService {}

impl MongodbCrudService<AddressTx> for AddressTxService {
    fn new() -> AddressTxService {
        AddressTxService {}
    }

    fn table(&self) -> Collection {
        collection(AddressTx::COLLECTION_NAME)
    }
}

impl AddressTxService {
    pub async fn monitor_address_tx(&self) -> Result<(), ApplicationError> {
        let addresses = ADDRESS_SERVICE.db_read_all_resources().await?;
        info!("addresses {:?}", addresses);
        for mut addr in addresses {
            let result = get_tx_by_address(&addr.address).await?;

            info!("Address {} has txs {:?}", addr.address, result);
            for tx in result {
                let resp2 = self.db_create_resource(
                    AddressTx {
                        id: None,
                        address: addr.address.to_owned(),
                        hash: tx.hash,
                    }).await;
                info!("saved tx {:?}", resp2.unwrap());
            }
        }
        Ok(())
    }
}