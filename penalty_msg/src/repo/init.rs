use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Error, MySql, Pool};

pub struct Repo {
    pub db: Pool<MySql>,
    pub cache: Option<i32>,
}

pub async fn init_repo() -> Result<Repo, Error> {
    // let pool = MySqlPool::connect("mysql://user:pass@host/database").await?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .min_connections(2)
        .connect("mysql://root:Nf_vTkdGpCoaaa8JPwsg@172.16.0.21:3306/fc_ld")
        .await?;

    let r = Repo {
        db: pool,
        cache: None,
    };

    return Ok(r);
}
