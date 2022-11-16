#[macro_use]
extern crate rocket;
extern crate serde_json;

use sqlx::mysql::MySqlPoolOptions;
use config::logger::init_logger;
mod router;
mod config;
mod model;

use config::Config;
// use model::db_pool::{DBpool};
use router::index;
use router::middleware::cors::{CORS};


// use std::env;

// fn rocket() -> _ {
    
//     // rocket::build().manage()
//     rocket::build().attach(DBpool::init()).mount("/", routes![index::read, index::index])
// }


#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = init_logger();
    // let database_url = env::var("DATABASE_URL")?;
    let database_url = "mysql://root:Nf_vTkdGpCoaaa8JPwsg@192.168.253.35:3306/fc_ld".to_string();

    // create a pgsql pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5).connect(&database_url).await?;
    
    let _rocket = rocket::build()
        .attach(CORS)
        .mount(
            Config::default().base,
            routes![index::read, index::index],
        )
        .manage(pool)
        .launch()
        .await
        .expect("something wrong happened at the launch!");
    Ok(())
}