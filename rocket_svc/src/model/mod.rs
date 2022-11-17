pub mod db_pool;
pub mod penalty_msg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIResponse<T: Serialize +Default> {
    pub code : i64,
    pub msg: String,
    pub val : T,
}

// impl<T: Default+Serialize> Default for  APIResponse<T> {
impl<T> Default for  APIResponse<T> 
where T: Default + Serialize
{

    fn default() -> Self {
        return Self{
            code: 0,
            msg:"ok".to_string(),
            val : T::default(),
        };
    }
    
}