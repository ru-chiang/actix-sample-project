use actix_web::{Error, http, HttpResponse, web};
use actix_web::dev::Service;
use reqwest::Response;


use crate::db::MongodbCrudService;

use super::{ADDRESS_BALANCE_SERVICE, AddressBalance};
use crate::utils::common::{RespResult, Resp};


pub async fn update_balance() -> RespResult {
    let res = ADDRESS_BALANCE_SERVICE.refresh_balance().await?;
    Resp::ok(res).to_json_result()
}

