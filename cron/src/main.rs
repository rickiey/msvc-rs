extern crate redis;
use redis::Commands;

fn main() {
    let client = redis::Client::open("redis://:Nf_vTkdGpCoaaa8JPwsg@172.16.0.21:6379/").unwrap();
    let mut con = client.get_connection().unwrap();
    // throw away the result, just make sure it does not fail
    let _ : () = con.set("my_key", 42).unwrap();
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let  rs :f64 = con.get("cache_linden_power_float").unwrap();
    println!("{:?}", rs);
}
