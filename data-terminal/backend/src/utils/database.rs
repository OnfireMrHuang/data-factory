use std::{collections::HashMap, sync::LazyLock};
use tokio::sync::RwLock;

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

    DB_POOL_MAP.write().await.insert(db.database.clone(), pool);
}

// Helper async function
async fn connect_db_and_return(database: String) -> Result<MySqlPool, sqlx::Error> {
    let db = &Setting::get().database;
    let url = format!(
        "mysql://{}:{}@{}:{}/{}?charset=utf8mb4&collation=utf8mb4_unicode_ci",
        db.user, db.password, db.host, db.port, database
    );
    Ok(MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(Some(std::time::Duration::from_secs(600)))
        .max_lifetime(Some(std::time::Duration::from_secs(1800)))
        .connect(&url)
        .await?)
}

// 获取全局配置库
pub async fn get_config_db() -> Result<MySqlPool, sqlx::Error> {
    let db = &Setting::get().database;
    let pool = DB_POOL_MAP.read().await[&db.database].clone();
    Ok(pool)
}

// 获取项目数据库
pub async fn get_project_db(_code: String) -> Result<MySqlPool, sqlx::Error> {

    // 先写死使用模版库
    let code = "template".to_string();


    let db_prefix = &Setting::get().database.prefix;
    let db_name = format!("{}{}", db_prefix, code);

    // First, check if pool exists, otherwise spawn blocking and await connection creation
    {
        let db_pool_map = DB_POOL_MAP.read().await;
        if let Some(pool) = db_pool_map.get(&db_name) {
            return Ok(pool.clone());
        }
    }

    let mut db_pool_map = DB_POOL_MAP.write().await;
    if let Some(pool) = db_pool_map.get(&db_name) {
        return Ok(pool.clone());
    }

    let db_name_clone = db_name.clone();
    let pool = connect_db_and_return(db_name_clone).await?;

    db_pool_map.insert(db_name.clone(), pool.clone());
    Ok(pool)
}
