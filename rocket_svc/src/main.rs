#[macro_use]
extern crate rocket;
extern crate serde_json;

use chrono::NaiveDateTime;
use rocket_db_pools::{Connection, Database};
use rocket_db_pools::sqlx;
use serde::{Deserialize, Serialize};

use config::logger::init_logger;

mod config;


#[derive(Database)]
#[database("db")]
struct DBpool(sqlx::SqlitePool);

#[get("/<from_addr>")]
async fn read(mut db: Connection<DBpool>, from_addr: String) -> String {
    let rows: Vec<PenaltyMsg> = sqlx::query_as::<_, PenaltyMsg>("select * from penalty_msgs where from_addr = ? limit 10").bind(&from_addr)
        .fetch_all(&mut *db).await.unwrap();
    // .and_then(|r| Ok(r))
    // .ok()

    log::info!("from_addr {}", from_addr);
    log::warn!("lens {}", rows.len());
    return serde_json::to_string(&rows).unwrap();
}

#[get("/hello")]
fn index() -> &'static str {
    log::info!("hello world");
    "{Hello,world!}"
}


#[launch]
fn rocket() -> _ {
    let _ = init_logger();
    rocket::build().attach(DBpool::init()).mount("/", routes![read, index])
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PenaltyMsg {
    height: i64,
    from_addr: String,
    to_addr: String,
    amount: String,
    // amount_v: Decimal,
    call_function: String,
    sub_cause: String,
    time_at: NaiveDateTime,
}
// fn main() {
//     rocket();
//     println!("Hello, world!");
// }
//
