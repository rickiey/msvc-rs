mod vectors;
// mod handle_err;
// use vectors::strings;
// use vectors::hashmap;

use vectors::struct_enum::{use_trait, My};
use vectors::struct_enum::MyTrait;
use vectors::lifetime::use_lftm;
use env_logger::Env;
// extern crate reqwest;


// use reqwest;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PenaltyMsg {
    pub to_addr: String,
    pub from_addr: String,
    pub height: i64,
    pub amount: String,
    pub time_at: String,
    pub call_function: String,
    pub sub_cause: String,
}

pub fn use_reqwest() -> Result<i32, reqwest::Error> {
    // let b =  blocking::get("http://localhost:9090/").text();
    //
    // println!("{:?}", b);

    // let body = reqwest::blocking::get("http://localhost:9090/")?.text()?;

    // let client = reqwest::Client::new();
    // let res = client.post("http://httpbin.org/post")
    //     .json(&map)
    //     .send()
    //     .await?;

    let p = PenaltyMsg{
        to_addr: "".to_string(),
        from_addr: "".to_string(),
        height: 0,
        amount: "".to_string(),
        time_at: "".to_string(),
        call_function: "".to_string(),
        sub_cause: "".to_string()
    };

    let cli = reqwest::blocking::Client::new();
    // let resp = reqwest::blocking::get("http://localhost:9090/");
    let resp = cli.post("http://112.13.172.72:59000/penalty_msg").json(&p).send();

    println!("body = {:?}", resp.unwrap().text().unwrap());

    Ok(4)
}

fn main() {
    // use_reqwest();
    println!("Hello, world!");
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    // strings::string_len();
    // hashmap::hash_map();
    // println!("{}", panic_handle::panics(5665).unwrap());


    use_trait();
    use_lftm();

    let a = My{ x: 5, y: 6564};

    a.my_method();

    let r = 1..=3;


    for i in r {
        println!("{}", i);
    }

    log::trace!("{}________done", "main");
    log::debug!("{}________done", "main");
    log::info!("{}________done", "main");
    log::warn!("{}________done", "main");
    log::error!("{}________done", "main");


}
