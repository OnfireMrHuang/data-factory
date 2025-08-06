use serde::{Deserialize, Serialize};

/// 队列类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QueueType {
    #[serde(rename = "kafka")]
    Kafka,
    #[serde(rename = "rabbitmq")]
    RabbitMQ,
    #[serde(rename = "redis")]
    Redis,
    #[serde(rename = "pulsar")]
    Pulsar,
    #[serde(rename = "rocketmq")]
    RocketMQ,
}

impl QueueType {
    /// 获取队列类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            QueueType::Kafka => "Apache Kafka",
            QueueType::RabbitMQ => "RabbitMQ",
            QueueType::Redis => "Redis",
            QueueType::Pulsar => "Apache Pulsar",
            QueueType::RocketMQ => "Apache RocketMQ",
        }
    }

    /// 获取默认端口
    pub fn default_port(&self) -> u16 {
        match self {
            QueueType::Kafka => 9092,
            QueueType::RabbitMQ => 5672,
            QueueType::Redis => 6379,
            QueueType::Pulsar => 6650,
            QueueType::RocketMQ => 9876,
        }
    }

    /// 获取管理端口
    pub fn admin_port(&self) -> u16 {
        match self {
            QueueType::Kafka => 8080,
            QueueType::RabbitMQ => 15672,
            QueueType::Redis => 6379,
            QueueType::Pulsar => 8080,
            QueueType::RocketMQ => 8080,
        }
    }
}

/// Topic信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicInfo {
    /// Topic名称
    pub name: String,
    /// 分区数量
    pub partitions: u32,
    /// 副本因子
    pub replication_factor: u32,
    /// 是否启用压缩
    pub compression_enabled: bool,
    /// 保留策略（天）
    pub retention_days: Option<u32>,
    /// 最大消息大小（字节）
    pub max_message_size: Option<u64>,
    /// 是否启用
    pub enabled: bool,
}

impl TopicInfo {
    /// 创建新的Topic
    pub fn new(
        name: String,
        partitions: u32,
        replication_factor: u32,
    ) -> Self {
        Self {
            name,
            partitions,
            replication_factor,
            compression_enabled: true,
            retention_days: Some(7),
            max_message_size: Some(1024 * 1024), // 1MB
            enabled: true
        }
    }

    /// 验证Topic配置
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
            && self.partitions > 0
            && self.replication_factor > 0
    }
}

/// 队列连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    /// 队列类型
    pub queue_type: QueueType,
    /// 主机地址
    pub host: String,
    /// 端口
    pub port: u16,
    /// 管理端口
    pub admin_port: u16,
    /// 用户名
    pub username: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 虚拟主机（RabbitMQ）
    pub virtual_host: Option<String>,
    /// 集群名称
    pub cluster_name: Option<String>,
    /// 是否启用SSL/TLS
    pub ssl_enabled: bool,
    /// 是否启用SASL认证
    pub sasl_enabled: bool,
    /// SASL机制
    pub sasl_mechanism: Option<String>,
    /// Topic列表
    pub topics: Vec<TopicInfo>,
}

impl QueueConfig {
    /// 创建新的队列配置
    pub fn new(
        queue_type: QueueType,
        host: String,
        port: Option<u16>,
    ) -> Self {
        let port = port.unwrap_or_else(|| queue_type.default_port());
        let admin_port = queue_type.admin_port();
        
        Self {
            queue_type,
            host,
            port,
            admin_port,
            username: None,
            password: None,
            virtual_host: None,
            cluster_name: None,
            ssl_enabled: false,
            sasl_enabled: false,
            sasl_mechanism: None,
            topics: Vec::new(),
        }
    }

    /// 获取连接URL
    pub fn connection_url(&self) -> String {
        let protocol = if self.ssl_enabled { "ssl" } else { "tcp" };
        format!("{}://{}:{}", protocol, self.host, self.port)
    }

    /// 获取管理URL
    pub fn admin_url(&self) -> String {
        let protocol = if self.ssl_enabled { "https" } else { "http" };
        format!("{}://{}:{}", protocol, self.host, self.admin_port)
    }

    /// 验证配置是否有效
    pub fn is_valid(&self) -> bool {
        !self.host.is_empty()
            && !self.host.is_empty()
            && self.port > 0
    }

    /// 添加Topic
    pub fn add_topic(&mut self, topic: TopicInfo) {
        self.topics.push(topic);
    }

    /// 移除Topic
    pub fn remove_topic(&mut self, name: &str) -> bool {
        let initial_len = self.topics.len();
        self.topics.retain(|topic| topic.name != name);
        let removed = initial_len != self.topics.len();
        removed
    }

    /// 获取Topic
    pub fn get_topic(&self, name: &str) -> Option<&TopicInfo> {
        self.topics.iter().find(|topic| topic.name == name)
    }

    /// 更新配置
    pub fn update(&mut self, updates: QueueConfigUpdate) {
        if let Some(host) = updates.host {
            self.host = host;
        }
        if let Some(port) = updates.port {
            self.port = port;
        }
        if let Some(admin_port) = updates.admin_port {
            self.admin_port = admin_port;
        }
        if let Some(username) = updates.username {
            self.username = Some(username);
        }
        if let Some(password) = updates.password {
            self.password = Some(password);
        }
        if let Some(virtual_host) = updates.virtual_host {
            self.virtual_host = Some(virtual_host);
        }
        if let Some(cluster_name) = updates.cluster_name {
            self.cluster_name = Some(cluster_name);
        }

        if let Some(ssl_enabled) = updates.ssl_enabled {
            self.ssl_enabled = ssl_enabled;
        }
        if let Some(sasl_enabled) = updates.sasl_enabled {
            self.sasl_enabled = sasl_enabled;
        }
        if let Some(sasl_mechanism) = updates.sasl_mechanism {
            self.sasl_mechanism = Some(sasl_mechanism);
        }

    }
}

/// 队列配置更新结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfigUpdate {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub admin_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub virtual_host: Option<String>,
    pub cluster_name: Option<String>,
    pub ssl_enabled: Option<bool>,
    pub sasl_enabled: Option<bool>,
    pub sasl_mechanism: Option<String>,
}

/// 队列连接测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueTestResult {
    pub success: bool,
    pub message: String,
    pub connection_time_ms: u64,
    pub broker_version: Option<String>,
    pub topic_count: Option<usize>,
    pub consumer_group_count: Option<usize>,
}

/// 队列配置列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfigList {
    pub configs: Vec<QueueConfig>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
}

/// 队列配置创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQueueConfigRequest {
    pub queue_type: QueueType,
    pub host: String,
    pub port: Option<u16>,
    pub admin_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub virtual_host: Option<String>,
    pub cluster_name: Option<String>,
    pub ssl_enabled: Option<bool>,
    pub sasl_enabled: Option<bool>,
    pub sasl_mechanism: Option<String>,
}

/// 队列统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    pub total_topics: usize,
    pub active_topics: usize,
    pub total_consumers: u64,
    pub total_producers: u64,
    pub messages_per_second: f64,
    pub bytes_per_second: u64,
    pub error_count: u64,
}

impl Default for QueueStats {
    fn default() -> Self {
        Self {
            total_topics: 0,
            active_topics: 0,
            total_consumers: 0,
            total_producers: 0,
            messages_per_second: 0.0,
            bytes_per_second: 0,
            error_count: 0,
        }
    }
}
