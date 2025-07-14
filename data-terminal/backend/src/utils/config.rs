use config::Config;
use config::{File, Environment};
use serde::Deserialize;
use once_cell::sync::OnceCell;

#[derive(Debug, Deserialize, Clone)]
pub struct Setting {
    pub database: Database,
    pub admin: Admin,
    pub jwt: Jwt,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub prefix: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Admin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwt {
    pub secret: String,
    pub expires: i64,
}

static SETTING: OnceCell<Setting> = OnceCell::new();

impl Setting {
    pub fn new() -> Result<Self, config::ConfigError> {
        let cfg = Config::builder()
            .add_source(File::with_name("backend/config/Setting")) // 自动查找 config.toml/json/yaml
            .add_source(Environment::with_prefix("APP")) // 读取 `APP_DATABASE_URL` 等变量
            .build()?;
        cfg.try_deserialize()
    }

    /// 初始化全局配置，main 调用
    pub fn init()  {
        let setting = Setting::new().expect("Unable to new Setting");
        SETTING.set(setting).map_err(|_| config::ConfigError::Message("Setting already initialized".into())).expect("Uable to set global setting");

    }

    /// 获取全局配置的引用
    pub fn get() -> &'static Setting {
        SETTING.get().expect("Setting is not initialized")
    }
}
