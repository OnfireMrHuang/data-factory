use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use once_cell::sync::OnceCell;
use crate::utils::config::Setting;

static DB_POOL: OnceCell<MySqlPool> = OnceCell::new();

pub async fn init() {
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
    DB_POOL.set(pool).expect("数据库池已初始化");
}

/// 获取全局数据库连接池
pub fn get_pool() -> &'static MySqlPool {
    DB_POOL.get().expect("数据库池未初始化")
}

