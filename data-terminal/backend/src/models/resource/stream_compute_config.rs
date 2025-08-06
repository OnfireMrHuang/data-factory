use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 流处理引擎类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamComputeType {
    #[serde(rename = "flink")]
    Flink,
    #[serde(rename = "kafka_streams")]
    KafkaStreams,
    #[serde(rename = "storm")]
    ApacheStorm,
    #[serde(rename = "samza")]
    ApacheSamza,
    #[serde(rename = "heron")]
    ApacheHeron,
}

impl StreamComputeType {
    /// 获取流处理引擎类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            StreamComputeType::Flink => "Apache Flink",
            StreamComputeType::KafkaStreams => "Kafka Streams",
            StreamComputeType::ApacheStorm => "Apache Storm",
            StreamComputeType::ApacheSamza => "Apache Samza",
            StreamComputeType::ApacheHeron => "Apache Heron",
        }
    }

    /// 获取默认端口
    pub fn default_port(&self) -> u16 {
        match self {
            StreamComputeType::Flink => 8081,
            StreamComputeType::KafkaStreams => 9092,
            StreamComputeType::ApacheStorm => 6627,
            StreamComputeType::ApacheSamza => 8080,
            StreamComputeType::ApacheHeron => 8080,
        }
    }

    /// 获取Web UI端口
    pub fn web_ui_port(&self) -> u16 {
        match self {
            StreamComputeType::Flink => 8081,
            StreamComputeType::KafkaStreams => 8080,
            StreamComputeType::ApacheStorm => 8080,
            StreamComputeType::ApacheSamza => 8080,
            StreamComputeType::ApacheHeron => 8080,
        }
    }
}

/// 流作业配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamJobConfig {
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
    pub resource_config: StreamResourceConfig,
    /// 检查点配置
    pub checkpoint_config: CheckpointConfig,
    /// 是否启用
    pub enabled: bool,
}

impl StreamJobConfig {
    /// 创建新的流作业配置
    pub fn new(name: String, job_type: String) -> Self {
        Self {
            name,
            job_type,
            main_class: None,
            jar_path: None,
            arguments: Vec::new(),
            environment_vars: HashMap::new(),
            resource_config: StreamResourceConfig::default(),
            checkpoint_config: CheckpointConfig::default(),
            enabled: true,
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

/// 流处理资源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamResourceConfig {
    /// 并行度
    pub parallelism: u32,
    /// 任务管理器数量
    pub task_manager_count: u32,
    /// 每个任务管理器的槽位数
    pub slots_per_task_manager: u32,
    /// 每个任务管理器的内存（MB）
    pub task_manager_memory_mb: u32,
    /// 作业管理器内存（MB）
    pub job_manager_memory_mb: u32,
    /// 最大执行时间（分钟）
    pub max_execution_time_minutes: u32,
}

impl Default for StreamResourceConfig {
    fn default() -> Self {
        Self {
            parallelism: 1,
            task_manager_count: 1,
            slots_per_task_manager: 1,
            task_manager_memory_mb: 1024,
            job_manager_memory_mb: 512,
            max_execution_time_minutes: 60,
        }
    }
}

/// 检查点配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointConfig {
    /// 检查点间隔（毫秒）
    pub checkpoint_interval_ms: u64,
    /// 检查点超时时间（毫秒）
    pub checkpoint_timeout_ms: u64,
    /// 最小暂停时间（毫秒）
    pub min_pause_between_checkpoints_ms: u64,
    /// 最大并发检查点数
    pub max_concurrent_checkpoints: u32,
    /// 外部化检查点
    pub externalized_checkpoints: bool,
    /// 检查点存储路径
    pub checkpoint_storage_path: Option<String>,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            checkpoint_interval_ms: 60000, // 1分钟
            checkpoint_timeout_ms: 600000, // 10分钟
            min_pause_between_checkpoints_ms: 10000, // 10秒
            max_concurrent_checkpoints: 1,
            externalized_checkpoints: false,
            checkpoint_storage_path: None,
        }
    }
}

/// 流处理连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamComputeConfig {
    /// 流处理引擎类型
    pub stream_compute_type: StreamComputeType,
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
    /// 是否启用SSL/TLS
    pub ssl_enabled: bool,
    /// 是否启用高可用
    pub high_availability_enabled: bool,
    /// 高可用存储路径
    pub ha_storage_path: Option<String>,
    /// 流作业配置列表
    pub stream_job_configs: Vec<StreamJobConfig>,
}

impl StreamComputeConfig {
    /// 创建新的流处理配置
    pub fn new(
        stream_compute_type: StreamComputeType,
        host: String,
        port: Option<u16>,
    ) -> Self {
        let port = port.unwrap_or_else(|| stream_compute_type.default_port());
        let web_ui_port = stream_compute_type.web_ui_port();
        
        Self {
            stream_compute_type,
            host,
            port,
            web_ui_port,
            username: None,
            password: None,
            cluster_name: None,
            ssl_enabled: false,
            high_availability_enabled: false,
            ha_storage_path: None,
            stream_job_configs: Vec::new(),
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
        !self.host.is_empty()
            && !self.host.is_empty()
            && self.port > 0
    }

    /// 添加流作业配置
    pub fn add_stream_job_config(&mut self, job_config: StreamJobConfig) {
        self.stream_job_configs.push(job_config);
    }

    /// 移除流作业配置
    pub fn remove_stream_job_config(&mut self, name: &str) -> bool {
        let initial_len = self.stream_job_configs.len();
        self.stream_job_configs.retain(|job| job.name != name);
        let removed = initial_len != self.stream_job_configs.len();
        removed
    }

    /// 获取流作业配置
    pub fn get_stream_job_config(&self, name: &str) -> Option<&StreamJobConfig> {
        self.stream_job_configs.iter().find(|job| job.name == name)
    }

    /// 更新配置
    pub fn update(&mut self, updates: StreamComputeConfigUpdate) {
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
        if let Some(ssl_enabled) = updates.ssl_enabled {
            self.ssl_enabled = ssl_enabled;
        }
        if let Some(high_availability_enabled) = updates.high_availability_enabled {
            self.high_availability_enabled = high_availability_enabled;
        }
        if let Some(ha_storage_path) = updates.ha_storage_path {
            self.ha_storage_path = Some(ha_storage_path);
        }
    }
}

/// 流处理配置更新结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamComputeConfigUpdate {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub web_ui_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub cluster_name: Option<String>,
    pub ssl_enabled: Option<bool>,
    pub high_availability_enabled: Option<bool>,
    pub ha_storage_path: Option<String>,
}

/// 流处理连接测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamComputeTestResult {
    pub success: bool,
    pub message: String,
    pub connection_time_ms: u64,
    pub cluster_version: Option<String>,
    pub active_jobs: Option<usize>,
    pub available_slots: Option<u32>,
}

/// 流处理配置列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamComputeConfigList {
    pub configs: Vec<StreamComputeConfig>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
}

/// 流处理配置创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStreamComputeConfigRequest {
    pub stream_compute_type: StreamComputeType,
    pub host: String,
    pub port: Option<u16>,
    pub web_ui_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub cluster_name: Option<String>,
    pub ssl_enabled: Option<bool>,
    pub high_availability_enabled: Option<bool>,
    pub ha_storage_path: Option<String>,
}

/// 流处理统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamComputeStats {
    pub total_jobs: usize,
    pub running_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
    pub total_task_managers: u32,
    pub active_task_managers: u32,
    pub total_slots: u32,
    pub available_slots: u32,
    pub total_memory_mb: u32,
    pub used_memory_mb: u32,
    pub error_count: u64,
}

impl Default for StreamComputeStats {
    fn default() -> Self {
        Self {
            total_jobs: 0,
            running_jobs: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            total_task_managers: 0,
            active_task_managers: 0,
            total_slots: 0,
            available_slots: 0,
            total_memory_mb: 0,
            used_memory_mb: 0,
            error_count: 0,
        }
    }
}
