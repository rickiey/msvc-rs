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
    use_reqwest();
    println!("Hello, world!");
}
