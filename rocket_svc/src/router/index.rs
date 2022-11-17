use sqlx::{Pool, MySql};
use rocket::{State, get, post};
use rocket::serde::json::Json;
use rocket::http::{Status};
use crate::model::APIResponse;
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
pub async fn insert( db: &State<Pool<MySql>>, pmsg: Json<PenaltyMsg>) -> Result<Json<APIResponse<u64>>, Status> {

    let res = PenaltyMsgSvc::create_pmsg(db, pmsg.0).await;

    // let mut s = APIResponse{
    //     code:0,
    //     msg: "".to_string(),
    //     val: 0,
    // };

    let mut s = APIResponse::default();
    match res {
        Ok(id) => {
            log::warn!("val ------------ {}", s.val);
            s.val = id;
            Ok(Json(s))
        },
        Err(e) => {
            log::error!("error : {}", e);
            Err(Status::NotFound)
        }
    }
}
