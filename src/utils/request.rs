use std::collections::HashMap;
use std::env;

use log::*;
use reqwest::Error;
use anyhow::anyhow;

use crate::utils::common::ApplicationError;

pub async fn get_balance_by_address(address: &str) -> Result<String, ApplicationError> {
    let api_key = env::var("API_KEY").expect("API_KEY env not set.");
    info!("address {:?}", address);
    let mut url: String = "https://api-ropsten.etherscan.io/api?module=account&action=balance&address=0x560aba705254C7db2D46Cb978B7D31Ee02DC5039&tag=latest&apikey=".to_owned();
    url.push_str(&api_key);
    let resp = reqwest::blocking::get(&url)?
        .json::<HashMap<String, String>>()?;

    Ok(resp.get("result").unwrap().to_owned())
}

impl From<reqwest::Error> for ApplicationError {
    fn from(e: Error) -> Self {
        log::error!("request error, {}", e.to_string());
        ApplicationError::InternalError { source: anyhow!(e) }
    }
}