use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 数据库类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DatabaseType {
    #[serde(rename = "mysql")]
    MySQL,
    #[serde(rename = "postgres")]
    PostgreSQL,
    #[serde(rename = "doris")]
    Doris,
}

impl DatabaseType {
    /// 获取数据库类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            DatabaseType::MySQL => "MySQL",
            DatabaseType::PostgreSQL => "PostgreSQL",
            DatabaseType::Doris => "Doris",
        }
    }

    /// 获取默认端口
    pub fn default_port(&self) -> u16 {
        match self {
            DatabaseType::MySQL => 3306,
            DatabaseType::PostgreSQL => 5432,
            DatabaseType::Doris => 9030,
        }
    }

    /// 获取连接URL前缀
    pub fn url_prefix(&self) -> &'static str {
        match self {
            DatabaseType::MySQL => "mysql://",
            DatabaseType::PostgreSQL => "postgresql://",
            DatabaseType::Doris => "mysql://", // Doris使用MySQL协议
        }
    }
}

/// 数据库连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 配置ID
    pub id: String,
    /// 配置名称
    pub name: String,
    /// 数据库类型
    pub database_type: DatabaseType,
    /// 主机地址
    pub host: String,
    /// 端口
    pub port: u16,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 数据库名称
    pub database: String,
    /// 连接池大小
    pub pool_size: u32,
    /// 连接超时时间（秒）
    pub connection_timeout: u64,
    /// 查询超时时间（秒）
    pub query_timeout: u64,
    /// 是否启用SSL
    pub ssl_enabled: bool,
    /// 额外连接参数
    pub extra_params: HashMap<String, String>,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl DatabaseConfig {
    /// 创建新的数据库配置
    pub fn new(
        id: String,
        name: String,
        database_type: DatabaseType,
        host: String,
        port: Option<u16>,
        username: String,
        password: String,
        database: String,
    ) -> Self {
        let port = port.unwrap_or_else(|| database_type.default_port());
        let now = chrono::Utc::now();
        
        Self {
            id,
            name,
            database_type,
            host,
            port,
            username,
            password,
            database,
            pool_size: 10,
            connection_timeout: 30,
            query_timeout: 60,
            ssl_enabled: false,
            extra_params: HashMap::new(),
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// 获取连接字符串
    pub fn connection_string(&self) -> String {
        let mut url = format!(
            "{}{}:{}@{}:{}/{}",
            self.database_type.url_prefix(),
            self.username,
            self.password,
            self.host,
            self.port,
            self.database
        );

        // 添加额外参数
        if !self.extra_params.is_empty() {
            let params: Vec<String> = self
                .extra_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str("?");
            url.push_str(&params.join("&"));
        }

        url
    }

    /// 获取DSN字符串（不包含密码）
    pub fn dsn_string(&self) -> String {
        format!(
            "{}{}@{}:{}/{}",
            self.database_type.url_prefix(),
            self.username,
            self.host,
            self.port,
            self.database
        )
    }

    /// 验证配置是否有效
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.name.is_empty()
            && !self.host.is_empty()
            && self.port > 0
            && !self.username.is_empty()
            && !self.database.is_empty()
    }

    /// 更新配置
    pub fn update(&mut self, updates: DatabaseConfigUpdate) {
        if let Some(name) = updates.name {
            self.name = name;
        }
        if let Some(host) = updates.host {
            self.host = host;
        }
        if let Some(port) = updates.port {
            self.port = port;
        }
        if let Some(username) = updates.username {
            self.username = username;
        }
        if let Some(password) = updates.password {
            self.password = password;
        }
        if let Some(database) = updates.database {
            self.database = database;
        }
        if let Some(pool_size) = updates.pool_size {
            self.pool_size = pool_size;
        }
        if let Some(connection_timeout) = updates.connection_timeout {
            self.connection_timeout = connection_timeout;
        }
        if let Some(query_timeout) = updates.query_timeout {
            self.query_timeout = query_timeout;
        }
        if let Some(ssl_enabled) = updates.ssl_enabled {
            self.ssl_enabled = ssl_enabled;
        }
        if let Some(enabled) = updates.enabled {
            self.enabled = enabled;
        }
        if let Some(extra_params) = updates.extra_params {
            self.extra_params = extra_params;
        }
        
        self.updated_at = chrono::Utc::now();
    }
}

/// 数据库配置更新结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfigUpdate {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub pool_size: Option<u32>,
    pub connection_timeout: Option<u64>,
    pub query_timeout: Option<u64>,
    pub ssl_enabled: Option<bool>,
    pub enabled: Option<bool>,
    pub extra_params: Option<HashMap<String, String>>,
}

/// 数据库连接测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub message: String,
    pub connection_time_ms: u64,
    pub server_version: Option<String>,
    pub database_count: Option<usize>,
}

/// 数据库列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub name: String,
    pub charset: Option<String>,
    pub collation: Option<String>,
    pub table_count: Option<usize>,
    pub size_bytes: Option<u64>,
}

/// 数据库配置列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfigList {
    pub configs: Vec<DatabaseConfig>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
}

/// 数据库配置创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDatabaseConfigRequest {
    pub name: String,
    pub database_type: DatabaseType,
    pub host: String,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
    pub database: String,
    pub pool_size: Option<u32>,
    pub connection_timeout: Option<u64>,
    pub query_timeout: Option<u64>,
    pub ssl_enabled: Option<bool>,
    pub extra_params: Option<HashMap<String, String>>,
}

/// 数据库配置更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDatabaseConfigRequest {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub pool_size: Option<u32>,
    pub connection_timeout: Option<u64>,
    pub query_timeout: Option<u64>,
    pub ssl_enabled: Option<bool>,
    pub enabled: Option<bool>,
    pub extra_params: Option<HashMap<String, String>>,
}

/// 数据库连接池配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    pub min_connections: u32,
    pub max_connections: u32,
    pub acquire_timeout: std::time::Duration,
    pub idle_timeout: std::time::Duration,
    pub max_lifetime: std::time::Duration,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 20,
            acquire_timeout: std::time::Duration::from_secs(30),
            idle_timeout: std::time::Duration::from_secs(600),
            max_lifetime: std::time::Duration::from_secs(1800),
        }
    }
}

/// 数据库统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub total_connections: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub connection_errors: u64,
    pub query_count: u64,
    pub avg_query_time_ms: f64,
}

impl Default for DatabaseStats {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            idle_connections: 0,
            connection_errors: 0,
            query_count: 0,
            avg_query_time_ms: 0.0,
        }
    }
}
