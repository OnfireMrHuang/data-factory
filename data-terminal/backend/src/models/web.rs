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

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Self {
            result: true,
            msg: "success".to_string(),
            data,
        }
    }

    pub fn error(msg: String) -> Self 
    where T: Default
    {
        Self {
            result: false,
            msg,
            data: Default::default(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PageQuery {
    pub keyword: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
