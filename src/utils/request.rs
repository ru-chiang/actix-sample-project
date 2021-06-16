use std::collections::HashMap;
use std::env;

use anyhow::anyhow;
use lazy_static::lazy_static;
use log::*;
use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::utils::common::ApplicationError;

lazy_static! {
    pub static ref API_KEY: String = env::var("API_KEY").expect("API_KEY env not set.");
}

pub async fn get_balance_by_address(address: &str) -> Result<String, ApplicationError> {
    info!("get_balance_by_address::address {:?}", address);
    let mut url: String = "https://api-ropsten.etherscan.io/api?module=account&action=balance&tag=latest".to_owned();
    url.push_str("&address=");
    url.push_str(address);
    url.push_str("&apikey=");
    url.push_str(&API_KEY);
    let resp = reqwest::blocking::get(&url)?
        .json::<HashMap<String, String>>()?;
    info!("get_balance_by_address::resp {:?}", resp);
    match resp.get("status").unwrap().as_ref() {
        "1" => Ok(resp.get("result").unwrap().to_owned()),
        _ => Err(ApplicationError::InternalError {
            source: anyhow!("request failed")
        }),
    }
}

#[derive(Deserialize, Debug)]
struct AddressTxResp {
    result: Vec<Tx>
}

#[derive(Deserialize, Debug)]
pub struct Tx {
    pub hash: String
}

pub async fn get_tx_by_address(address: &str) -> Result<Vec<Tx>, ApplicationError> {
    info!("address {:?}", address);
    let mut url: String = "https://api-ropsten.etherscan.io/api?module=account&action=txlist&startblock=0&endblock=99999999&sort=asc".to_owned();
    url.push_str("&address=");
    url.push_str(address);
    url.push_str("&apikey=");
    url.push_str(&API_KEY);
    let resp = reqwest::blocking::get(&url)?.json::<AddressTxResp>()?;
    info!("get_tx_by_address::resp {:?}", resp);
    Ok(resp.result)
}


impl From<reqwest::Error> for ApplicationError {
    fn from(e: Error) -> Self {
        log::error!("request error, {}", e.to_string());
        ApplicationError::InternalError { source: anyhow!(e) }
    }
}


// #[actix_rt::test]
// async fn test_call_address_tx() {
//     use std::env;
//     let address = "0x7c1ce6a008ef40c13e4eb144a6cc74f0e0aeac7e";
//     let res = get_tx_by_address(address).await;
//     println!("res {}", res.unwrap())
// }
