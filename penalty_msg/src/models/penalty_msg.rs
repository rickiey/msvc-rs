use  chrono::{NaiveDateTime,NaiveDate};
use rust_decimal::Decimal;
use sqlx;
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PenaltyMsg {
    id:u64,
    height: u64,
    from_addr: String,
    to_addr: String,
    amount: String,
    amount_v: Decimal,
    call_function: String,
    sub_cause: String,
    time_at: NaiveDateTime,
}

impl PenaltyMsg {
    pub fn new() -> PenaltyMsg {
        let tm = NaiveDate::from_ymd(2022,1,1).and_hms(0,0,0);
        return PenaltyMsg {
            id: 0,
            height: 0,
            from_addr: String::new(),
            to_addr: String::new(),
            amount: String::new(),
            amount_v: Decimal::from(0),
            call_function: String::new(),
            sub_cause: String::new(),
            // time_at: NaiveDateTime::from_str("2021-10-28 17:40:00").unwrap(),
            time_at: tm,
        };
    }
}
