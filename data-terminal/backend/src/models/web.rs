use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub result: bool,
    pub msg: String,
    pub data: T,
}

