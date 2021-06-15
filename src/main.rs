#[macro_use]
extern crate bson;

use std::env;

use actix_web::{App, HttpServer, web};


mod utils;
mod address;
mod address_balance;
mod db;

use utils::logging::{init_logger};

fn get_binding_address() -> String {
    let port = env::var("PORT").expect("PORT env not set.");
    "127.0.0.1:".to_owned() + &port
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    init_logger();
    let binding_address = get_binding_address();
    HttpServer::new(|| App::new()
        .service(
            web::scope("/address")
                .route("", web::post().to(address::save))
                .route("", web::get().to(address::get_all))
                .route("{id}", web::get().to(address::get))
                .route("{id}", web::post().to(address::update))
                .route("{id}", web::delete().to(address::remove))
        )
        .service(
            web::scope("/address_balance")
                .route("", web::get().to(address_balance::update_balance))
        )
    )
        .bind(&binding_address)
        .expect(&format!("Can not bind to {}", &binding_address))
        .run()
        .await
}
