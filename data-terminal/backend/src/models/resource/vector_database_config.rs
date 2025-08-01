use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 向量数据库类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VectorDatabaseType {
    #[serde(rename = "milvus")]
    Milvus,
    #[serde(rename = "weaviate")]
    Weaviate,
    #[serde(rename = "qdrant")]
    Qdrant,
    #[serde(rename = "pinecone")]
    Pinecone,
    #[serde(rename = "chroma")]
    Chroma,
    #[serde(rename = "elasticsearch")]
    Elasticsearch,
}

impl VectorDatabaseType {
    /// 获取向量数据库类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            VectorDatabaseType::Milvus => "Milvus",
            VectorDatabaseType::Weaviate => "Weaviate",
            VectorDatabaseType::Qdrant => "Qdrant",
            VectorDatabaseType::Pinecone => "Pinecone",
            VectorDatabaseType::Chroma => "Chroma",
            VectorDatabaseType::Elasticsearch => "Elasticsearch",
        }
    }

    /// 获取默认端口
    pub fn default_port(&self) -> u16 {
        match self {
            VectorDatabaseType::Milvus => 19530,
            VectorDatabaseType::Weaviate => 8080,
            VectorDatabaseType::Qdrant => 6333,
            VectorDatabaseType::Pinecone => 443,
            VectorDatabaseType::Chroma => 8000,
            VectorDatabaseType::Elasticsearch => 9200,
        }
    }

    /// 获取Web UI端口
    pub fn web_ui_port(&self) -> u16 {
        match self {
            VectorDatabaseType::Milvus => 9091,
            VectorDatabaseType::Weaviate => 8080,
            VectorDatabaseType::Qdrant => 6333,
            VectorDatabaseType::Pinecone => 443,
            VectorDatabaseType::Chroma => 8000,
            VectorDatabaseType::Elasticsearch => 9200,
        }
    }
}

/// 集合配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    /// 集合名称
    pub name: String,
    /// 向量维度
    pub vector_dimension: u32,
    /// 向量类型
    pub vector_type: String,
    /// 距离度量方式
    pub distance_metric: String,
    /// 索引类型
    pub index_type: String,
    /// 索引参数
    pub index_params: HashMap<String, String>,
    /// 分片数量
    pub shard_count: u32,
    /// 副本数量
    pub replica_count: u32,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CollectionConfig {
    /// 创建新的集合配置
    pub fn new(
        name: String,
        vector_dimension: u32,
        vector_type: String,
        distance_metric: String,
    ) -> Self {
        Self {
            name,
            vector_dimension,
            vector_type,
            distance_metric,
            index_type: "IVF_FLAT".to_string(),
            index_params: HashMap::new(),
            shard_count: 1,
            replica_count: 1,
            enabled: true,
            created_at: chrono::Utc::now(),
        }
    }

    /// 设置索引参数
    pub fn set_index_param(&mut self, key: String, value: String) {
        self.index_params.insert(key, value);
    }

    /// 验证集合配置
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
            && self.vector_dimension > 0
            && !self.vector_type.is_empty()
            && !self.distance_metric.is_empty()
    }
}

/// 向量数据库连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseConfig {
    /// 配置ID
    pub id: String,
    /// 配置名称
    pub name: String,
    /// 向量数据库类型
    pub vector_database_type: VectorDatabaseType,
    /// 主机地址
    pub host: String,
    /// 端口
    pub port: u16,
    /// Web UI端口
    pub web_ui_port: u16,
    /// 用户名
    pub username: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// API密钥（用于Pinecone等）
    pub api_key: Option<String>,
    /// 环境（用于Pinecone等）
    pub environment: Option<String>,
    /// 数据库名称
    pub database: Option<String>,
    /// 连接超时时间（秒）
    pub connection_timeout: u64,
    /// 查询超时时间（秒）
    pub query_timeout: u64,
    /// 是否启用SSL/TLS
    pub ssl_enabled: bool,
    /// 是否启用认证
    pub auth_enabled: bool,
    /// 认证类型
    pub auth_type: Option<String>,
    /// 额外连接参数
    pub extra_params: HashMap<String, String>,
    /// 集合配置列表
    pub collections: Vec<CollectionConfig>,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl VectorDatabaseConfig {
    /// 创建新的向量数据库配置
    pub fn new(
        id: String,
        name: String,
        vector_database_type: VectorDatabaseType,
        host: String,
        port: Option<u16>,
    ) -> Self {
        let port = port.unwrap_or_else(|| vector_database_type.default_port());
        let web_ui_port = vector_database_type.web_ui_port();
        let now = chrono::Utc::now();
        
        Self {
            id,
            name,
            vector_database_type,
            host,
            port,
            web_ui_port,
            username: None,
            password: None,
            api_key: None,
            environment: None,
            database: None,
            connection_timeout: 30,
            query_timeout: 60,
            ssl_enabled: false,
            auth_enabled: false,
            auth_type: None,
            extra_params: HashMap::new(),
            collections: Vec::new(),
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// 获取连接URL
    pub fn connection_url(&self) -> String {
        let protocol = if self.ssl_enabled { "https" } else { "http" };
        format!("{}://{}:{}", protocol, self.host, self.port)
    }

    /// 获取Web UI URL
    pub fn web_ui_url(&self) -> String {
        let protocol = if self.ssl_enabled { "https" } else { "http" };
        format!("{}://{}:{}", protocol, self.host, self.web_ui_port)
    }

    /// 验证配置是否有效
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.name.is_empty()
            && !self.host.is_empty()
            && self.port > 0
    }

    /// 添加集合配置
    pub fn add_collection(&mut self, collection: CollectionConfig) {
        self.collections.push(collection);
        self.updated_at = chrono::Utc::now();
    }

    /// 移除集合配置
    pub fn remove_collection(&mut self, name: &str) -> bool {
        let initial_len = self.collections.len();
        self.collections.retain(|collection| collection.name != name);
        let removed = initial_len != self.collections.len();
        if removed {
            self.updated_at = chrono::Utc::now();
        }
        removed
    }

    /// 获取集合配置
    pub fn get_collection(&self, name: &str) -> Option<&CollectionConfig> {
        self.collections.iter().find(|collection| collection.name == name)
    }

    /// 更新配置
    pub fn update(&mut self, updates: VectorDatabaseConfigUpdate) {
        if let Some(name) = updates.name {
            self.name = name;
        }
        if let Some(host) = updates.host {
            self.host = host;
        }
        if let Some(port) = updates.port {
            self.port = port;
        }
        if let Some(web_ui_port) = updates.web_ui_port {
            self.web_ui_port = web_ui_port;
        }
        if let Some(username) = updates.username {
            self.username = Some(username);
        }
        if let Some(password) = updates.password {
            self.password = Some(password);
        }
        if let Some(api_key) = updates.api_key {
            self.api_key = Some(api_key);
        }
        if let Some(environment) = updates.environment {
            self.environment = Some(environment);
        }
        if let Some(database) = updates.database {
            self.database = Some(database);
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
        if let Some(auth_enabled) = updates.auth_enabled {
            self.auth_enabled = auth_enabled;
        }
        if let Some(auth_type) = updates.auth_type {
            self.auth_type = Some(auth_type);
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

/// 向量数据库配置更新结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseConfigUpdate {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub web_ui_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub api_key: Option<String>,
    pub environment: Option<String>,
    pub database: Option<String>,
    pub connection_timeout: Option<u64>,
    pub query_timeout: Option<u64>,
    pub ssl_enabled: Option<bool>,
    pub auth_enabled: Option<bool>,
    pub auth_type: Option<String>,
    pub enabled: Option<bool>,
    pub extra_params: Option<HashMap<String, String>>,
}

/// 向量数据库连接测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseTestResult {
    pub success: bool,
    pub message: String,
    pub connection_time_ms: u64,
    pub server_version: Option<String>,
    pub collection_count: Option<usize>,
    pub total_vectors: Option<u64>,
}

/// 向量数据库配置列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseConfigList {
    pub configs: Vec<VectorDatabaseConfig>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
}

/// 向量数据库配置创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVectorDatabaseConfigRequest {
    pub name: String,
    pub vector_database_type: VectorDatabaseType,
    pub host: String,
    pub port: Option<u16>,
    pub web_ui_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub api_key: Option<String>,
    pub environment: Option<String>,
    pub database: Option<String>,
    pub connection_timeout: Option<u64>,
    pub query_timeout: Option<u64>,
    pub ssl_enabled: Option<bool>,
    pub auth_enabled: Option<bool>,
    pub auth_type: Option<String>,
    pub extra_params: Option<HashMap<String, String>>,
}

/// 向量数据库统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseStats {
    pub total_collections: usize,
    pub active_collections: usize,
    pub total_vectors: u64,
    pub total_indexes: u32,
    pub query_count: u64,
    pub insert_count: u64,
    pub delete_count: u64,
    pub error_count: u64,
}

impl Default for VectorDatabaseStats {
    fn default() -> Self {
        Self {
            total_collections: 0,
            active_collections: 0,
            total_vectors: 0,
            total_indexes: 0,
            query_count: 0,
            insert_count: 0,
            delete_count: 0,
            error_count: 0,
        }
    }
}
