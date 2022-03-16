
mod config;
mod controller;
mod models;
mod repo;

use chrono::format::Pad;
use models::penalty_msg;
use sqlx;
use sqlx::{Error, MySql, Pool};
use crate::penalty_msg::PenaltyMsg;
use actix_web::{web, App, HttpServer,};
use controller::route;
use actix_web;


#[actix_web::main]
async fn main() ->  std::io::Result<()> {
    let repos = repo::init::init_repo().await.unwrap();

    let  pmsg =PenaltyMsg::query_by_miner(repos, String::from("f01103850")).await;

    for i in pmsg {
        println!("{:?}", i);
    }

    HttpServer::new(|| {
        App::new()
            .service(route::hello)
            .service(route::echo)
            .route("/hey", web::get().to(route::manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
