use crate::model::penalty_msg::PenaltyMsg;
use sqlx::{Pool, MySql};


pub struct PenaltyMsgSvc{}

impl PenaltyMsgSvc {
    pub async fn select_all(db :&Pool<MySql>, from_addr : String) -> Result<Vec<PenaltyMsg>, sqlx::Error> {
        let rows: Vec<PenaltyMsg> = sqlx::query_as::<_, PenaltyMsg>("select * from penalty_msg where from_addr = ? limit 10").bind(&from_addr)
        .fetch_all(db).await?;
        return Ok(rows);
    }

    pub async fn create_pmsg(db :&Pool<MySql>, pmsg:PenaltyMsg) -> Result<u64, sqlx::Error> {
        log::info!("height {:?}", pmsg);
        let _res = sqlx::query("insert into penalty_msg (height,from_addr, to_addr, time_at) values (?, ?, ?, ?) ")
            .bind(pmsg.height).bind(pmsg.from_addr).bind(pmsg.to_addr).bind(pmsg.time_at)
            .execute(db)
            .await?.last_insert_id();
        // 
        
        
        Ok(_res)
    }
}

