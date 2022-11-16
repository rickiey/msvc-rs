pub mod logger;



pub struct Config {
    pub base: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base: "/api/v1/".to_string(),
        }
    }
}
