mod vectors;
// mod handle_err;
// use vectors::strings;
// use vectors::hashmap;

use vectors::struct_enum::{use_trait, My};
use vectors::struct_enum::MyTrait;
use vectors::lifetime::use_lftm;
use env_logger::Env;
extern crate log;

fn main() {
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
