use std::{collections::HashMap, sync::{RwLock, LazyLock}};

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use crate::utils::config::Setting;

static DB_POOL_MAP: LazyLock<RwLock<HashMap<String, MySqlPool>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn config_db_init() {
    let db = &Setting::get().database;
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.database
    );
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .expect("数据库连接失败");

    DB_POOL_MAP.write().unwrap().insert(db.database.clone(), pool);
}

// 获取全局配置库
pub fn get_config_db() -> MySqlPool {
    let db = &Setting::get().database;
    DB_POOL_MAP.read().unwrap()[&db.database].clone()
}

// 获取项目数据库
pub fn get_project_db(code: String) -> MySqlPool {
    let db_prefix = &Setting::get().database.prefix;
    let db_name = format!("{}_{}", db_prefix, code);
    DB_POOL_MAP.read().unwrap()[&db_name].clone()
}
