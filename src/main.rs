#[macro_use]
extern crate bson;

use std::env;
use std::thread;
use std::time::Duration;

use actix_web::{App, HttpServer, web};
use futures::executor;
use job_scheduler::{Job, JobScheduler};
use log::*;

use utils::logging::init_logger;

use crate::address_tx::ADDRESS_TX_SERVICE;

mod utils;
mod address;
mod address_balance;
mod db;
mod address_tx;

fn get_binding_address() -> String {
    let port = env::var("PORT").expect("PORT env not set.");
    "127.0.0.1:".to_owned() + &port
}

fn monitor_transaction_for_address() {
    let mut sched = JobScheduler::new();
    let cron_expression = env::var("MONITOR_TX_CRON")
        .expect("MONITOR_TX_CRON env not set.");
    sched.add(Job::new(cron_expression.parse().unwrap(), || {
        info!("monitor_transaction_for_address");
        executor::block_on(ADDRESS_TX_SERVICE.monitor_address_tx());
    }));
    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    init_logger();
    let binding_address = get_binding_address();


    // start cron
    thread::Builder::new().name("worker".to_string()).spawn(move || {
        info!("starting cron");
        monitor_transaction_for_address()
    });

    // start http server
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
