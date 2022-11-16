use sqlx::{Pool, MySql};
use rocket::{State, get};
use rocket::serde::json::Json;

use rocket::http::{Status};


use crate::model::penalty_msg::PenaltyMsg;


#[get("/hello")]
pub fn index() -> &'static str {
    log::info!("hello world");
    "{\"Hello,\":\"world!\"}"
}



#[get("/<from_addr>")]
pub async fn read( db: &State<Pool<MySql>>, from_addr: String) -> Result<Json<Vec<PenaltyMsg>>,Status> {
    let rows: Vec<PenaltyMsg> = sqlx::query_as::<_, PenaltyMsg>("select * from penalty_msgs where from_addr = ? limit 10").bind(&from_addr)
        .fetch_all(&**db).await.unwrap();
    // .and_then(|r| Ok(r))
    // .ok()

    log::info!("from_addr {}", from_addr);
    log::warn!("lens {}", rows.len());
    return Ok(Json(rows));
}

