use actix_web::{Error, http, HttpResponse, web};
use actix_web::dev::Service;
use reqwest::Response;

use crate::db::MongodbCrudService;
use crate::utils::common::{Resp, ResponseResult};

use super::{ADDRESS_BALANCE_SERVICE, AddressBalance};

pub async fn update_balance() -> ResponseResult {
    let res = ADDRESS_BALANCE_SERVICE.refresh_balance().await?;
    Resp::ok(res).to_json_result()
}

