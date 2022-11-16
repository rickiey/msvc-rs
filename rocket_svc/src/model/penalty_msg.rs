
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


// create table penalty_msgs (height int, from_addr text, to_addr text,amount text,call_function text,sub_cause text, time_at TEXT )
/*
insert into penalty_msgs values(233,"asd","qwe","0.232354564","call","sub","2020-05-05 14:12:23");

 curl -s localhost:8000/asd | jq
*/
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PenaltyMsg {
    pub height: u64,
    from_addr: String,
    to_addr: String,
    amount: String,
    // amount_v: Decimal,
    call_function: String,
    sub_cause: String,
    time_at: NaiveDateTime,
}
