use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 批处理引擎类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BatchComputeType {
    #[serde(rename = "spark")]
    Spark,
    #[serde(rename = "hadoop")]
    Hadoop,
    #[serde(rename = "flink_batch")]
    FlinkBatch,
    #[serde(rename = "beam")]
    ApacheBeam,
    #[serde(rename = "airflow")]
    Airflow,
}

impl BatchComputeType {
    /// 获取批处理引擎类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            BatchComputeType::Spark => "Apache Spark",
            BatchComputeType::Hadoop => "Apache Hadoop",
            BatchComputeType::FlinkBatch => "Apache Flink (Batch)",
            BatchComputeType::ApacheBeam => "Apache Beam",
            BatchComputeType::Airflow => "Apache Airflow",
        }
    }

    /// 获取默认端口
    pub fn default_port(&self) -> u16 {
        match self {
            BatchComputeType::Spark => 7077,
            BatchComputeType::Hadoop => 9000,
            BatchComputeType::FlinkBatch => 8081,
            BatchComputeType::ApacheBeam => 8080,
            BatchComputeType::Airflow => 8080,
        }
    }

    /// 获取Web UI端口
    pub fn web_ui_port(&self) -> u16 {
        match self {
            BatchComputeType::Spark => 8080,
            BatchComputeType::Hadoop => 9870,
            BatchComputeType::FlinkBatch => 8081,
            BatchComputeType::ApacheBeam => 8080,
            BatchComputeType::Airflow => 8080,
        }
    }
}

/// 作业配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobConfig {
    /// 作业名称
    pub name: String,
    /// 作业类型
    pub job_type: String,
    /// 主类名
    pub main_class: Option<String>,
    /// JAR文件路径
    pub jar_path: Option<String>,
    /// 参数列表
    pub arguments: Vec<String>,
    /// 环境变量
    pub environment_vars: HashMap<String, String>,
    /// 资源配置
    pub resource_config: ResourceConfig,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl JobConfig {
    /// 创建新的作业配置
    pub fn new(name: String, job_type: String) -> Self {
        Self {
            name,
            job_type,
            main_class: None,
            jar_path: None,
            arguments: Vec::new(),
            environment_vars: HashMap::new(),
            resource_config: ResourceConfig::default(),
            enabled: true,
            created_at: chrono::Utc::now(),
        }
    }

    /// 添加参数
    pub fn add_argument(&mut self, arg: String) {
        self.arguments.push(arg);
    }

    /// 设置环境变量
    pub fn set_env_var(&mut self, key: String, value: String) {
        self.environment_vars.insert(key, value);
    }

    /// 验证作业配置
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.job_type.is_empty()
    }
}

/// 资源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    /// 执行器数量
    pub executor_count: u32,
    /// 每个执行器的核心数
    pub executor_cores: u32,
    /// 每个执行器的内存（MB）
    pub executor_memory_mb: u32,
    /// 驱动程序内存（MB）
    pub driver_memory_mb: u32,
    /// 驱动程序核心数
    pub driver_cores: u32,
    /// 最大执行时间（分钟）
    pub max_execution_time_minutes: u32,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            executor_count: 2,
            executor_cores: 2,
            executor_memory_mb: 1024,
            driver_memory_mb: 512,
            driver_cores: 1,
            max_execution_time_minutes: 60,
        }
    }
}

/// 批处理连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchComputeConfig {
    /// 配置ID
    pub id: String,
    /// 配置名称
    pub name: String,
    /// 批处理引擎类型
    pub batch_compute_type: BatchComputeType,
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
    /// 集群名称
    pub cluster_name: Option<String>,
    /// 连接超时时间（秒）
    pub connection_timeout: u64,
    /// 作业提交超时时间（秒）
    pub job_submit_timeout: u64,
    /// 是否启用SSL/TLS
    pub ssl_enabled: bool,
    /// 是否启用Kerberos认证
    pub kerberos_enabled: bool,
    /// Kerberos主体
    pub kerberos_principal: Option<String>,
    /// 额外连接参数
    pub extra_params: HashMap<String, String>,
    /// 作业配置列表
    pub job_configs: Vec<JobConfig>,
    /// 是否启用
    pub enabled: bool,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl BatchComputeConfig {
    /// 创建新的批处理配置
    pub fn new(
        id: String,
        name: String,
        batch_compute_type: BatchComputeType,
        host: String,
        port: Option<u16>,
    ) -> Self {
        let port = port.unwrap_or_else(|| batch_compute_type.default_port());
        let web_ui_port = batch_compute_type.web_ui_port();
        let now = chrono::Utc::now();
        
        Self {
            id,
            name,
            batch_compute_type,
            host,
            port,
            web_ui_port,
            username: None,
            password: None,
            cluster_name: None,
            connection_timeout: 30,
            job_submit_timeout: 300,
            ssl_enabled: false,
            kerberos_enabled: false,
            kerberos_principal: None,
            extra_params: HashMap::new(),
            job_configs: Vec::new(),
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

    /// 添加作业配置
    pub fn add_job_config(&mut self, job_config: JobConfig) {
        self.job_configs.push(job_config);
        self.updated_at = chrono::Utc::now();
    }

    /// 移除作业配置
    pub fn remove_job_config(&mut self, name: &str) -> bool {
        let initial_len = self.job_configs.len();
        self.job_configs.retain(|job| job.name != name);
        let removed = initial_len != self.job_configs.len();
        if removed {
            self.updated_at = chrono::Utc::now();
        }
        removed
    }

    /// 获取作业配置
    pub fn get_job_config(&self, name: &str) -> Option<&JobConfig> {
        self.job_configs.iter().find(|job| job.name == name)
    }

    /// 更新配置
    pub fn update(&mut self, updates: BatchComputeConfigUpdate) {
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
        if let Some(cluster_name) = updates.cluster_name {
            self.cluster_name = Some(cluster_name);
        }
        if let Some(connection_timeout) = updates.connection_timeout {
            self.connection_timeout = connection_timeout;
        }
        if let Some(job_submit_timeout) = updates.job_submit_timeout {
            self.job_submit_timeout = job_submit_timeout;
        }
        if let Some(ssl_enabled) = updates.ssl_enabled {
            self.ssl_enabled = ssl_enabled;
        }
        if let Some(kerberos_enabled) = updates.kerberos_enabled {
            self.kerberos_enabled = kerberos_enabled;
        }
        if let Some(kerberos_principal) = updates.kerberos_principal {
            self.kerberos_principal = Some(kerberos_principal);
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

/// 批处理配置更新结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchComputeConfigUpdate {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub web_ui_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub cluster_name: Option<String>,
    pub connection_timeout: Option<u64>,
    pub job_submit_timeout: Option<u64>,
    pub ssl_enabled: Option<bool>,
    pub kerberos_enabled: Option<bool>,
    pub kerberos_principal: Option<String>,
    pub enabled: Option<bool>,
    pub extra_params: Option<HashMap<String, String>>,
}

/// 批处理连接测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchComputeTestResult {
    pub success: bool,
    pub message: String,
    pub connection_time_ms: u64,
    pub cluster_version: Option<String>,
    pub active_jobs: Option<usize>,
    pub available_resources: Option<String>,
}

/// 批处理配置列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchComputeConfigList {
    pub configs: Vec<BatchComputeConfig>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
}

/// 批处理配置创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatchComputeConfigRequest {
    pub name: String,
    pub batch_compute_type: BatchComputeType,
    pub host: String,
    pub port: Option<u16>,
    pub web_ui_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub cluster_name: Option<String>,
    pub connection_timeout: Option<u64>,
    pub job_submit_timeout: Option<u64>,
    pub ssl_enabled: Option<bool>,
    pub kerberos_enabled: Option<bool>,
    pub kerberos_principal: Option<String>,
    pub extra_params: Option<HashMap<String, String>>,
}

/// 批处理统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchComputeStats {
    pub total_jobs: usize,
    pub running_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
    pub total_executors: u32,
    pub active_executors: u32,
    pub total_memory_mb: u32,
    pub used_memory_mb: u32,
    pub error_count: u64,
}

impl Default for BatchComputeStats {
    fn default() -> Self {
        Self {
            total_jobs: 0,
            running_jobs: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            total_executors: 0,
            active_executors: 0,
            total_memory_mb: 0,
            used_memory_mb: 0,
            error_count: 0,
        }
    }
}
