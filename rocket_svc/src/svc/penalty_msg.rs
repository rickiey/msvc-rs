use crate::model::penalty_msg::PenaltyMsg;
use sqlx::{Pool, MySql};


pub struct PenaltyMsgSvc{}

impl PenaltyMsgSvc {
    pub async fn select_all(db :&Pool<MySql>, from_addr : String) -> Result<Vec<PenaltyMsg>, sqlx::Error> {
        let rows: Vec<PenaltyMsg> = sqlx::query_as::<_, PenaltyMsg>("select * from penalty_msgs where from_addr = ? limit 10").bind(&from_addr)
        .fetch_all(db).await?;
        return Ok(rows);
    }

    pub async fn create_pmsg(db :&Pool<MySql>, pmsg:PenaltyMsg) -> Result<(), sqlx::Error> {
        sqlx::query("insert into penalty_msgs (height) values ($1) ")
            .bind(pmsg.height)
            .execute(db)
            .await?;
        Ok(())
    }
}

