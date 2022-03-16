
use crate::models::penalty_msg::PenaltyMsg;

use sqlx;
use crate::repo::init::Repo;


impl PenaltyMsg {
    pub async fn query_by_miner(r: Repo, miner : String) -> Result<Vec<PenaltyMsg>, sqlx::Error> {
        let  pmsg = sqlx::query_as::<_, PenaltyMsg>("select * from penalty_msg where from_addr = ?")
            .bind(miner)
            .fetch_all(&r.db);
        return pmsg.await;
    }

}
