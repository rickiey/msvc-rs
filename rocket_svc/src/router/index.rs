use sqlx::{Pool, MySql};
use rocket::{State, get, post};
use rocket::serde::json::Json;
use rocket::http::{Status};
use crate::svc::penalty_msg::PenaltyMsgSvc;
use crate::model::penalty_msg::PenaltyMsg;

#[get("/hello")]
pub fn index() -> &'static str {
    log::info!("hello world");
    "{\"Hello,\":\"world!\"}"
}



#[get("/<from_addr>")]
pub async fn read( db: &State<Pool<MySql>>, from_addr: String) -> Result<Json<Vec<PenaltyMsg>>, Status> {

    let res = PenaltyMsgSvc::select_all(db, from_addr).await;

    match res {
        Ok(re) => Ok(Json(re)),
        _ => Err(Status::NotFound),
    }
}

#[post("/penalty-msg", data = "<pmsg>")]
pub async fn insert( db: &State<Pool<MySql>>, pmsg: Json<PenaltyMsg>) -> Result<(), Status> {

    let res = PenaltyMsgSvc::create_pmsg(db, pmsg.0).await;

    match res {
        Ok(_) => Ok(()),
        _ => Err(Status::NotFound),
    }
}
