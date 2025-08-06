use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 文件系统类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileSystemType {
    #[serde(rename = "hdfs")]
    HDFS,
    #[serde(rename = "s3")]
    S3,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "ftp")]
    FTP,
    #[serde(rename = "sftp")]
    SFTP,
}

impl FileSystemType {
    /// 获取文件系统类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            FileSystemType::HDFS => "HDFS",
            FileSystemType::S3 => "Amazon S3",
            FileSystemType::Local => "Local File System",
            FileSystemType::FTP => "FTP",
            FileSystemType::SFTP => "SFTP",
        }
    }

    /// 获取默认端口
    pub fn default_port(&self) -> u16 {
        match self {
            FileSystemType::HDFS => 9000,
            FileSystemType::S3 => 443,
            FileSystemType::Local => 0, // 本地文件系统不需要端口
            FileSystemType::FTP => 21,
            FileSystemType::SFTP => 22,
        }
    }

    /// 是否需要认证
    pub fn requires_auth(&self) -> bool {
        match self {
            FileSystemType::HDFS => true,
            FileSystemType::S3 => true,
            FileSystemType::Local => false,
            FileSystemType::FTP => true,
            FileSystemType::SFTP => true,
        }
    }
}

/// 挂载目录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountDirectory {
    /// 挂载点名称
    pub name: String,
    /// 远程路径
    pub remote_path: String,
    /// 本地挂载路径
    pub local_path: String,
    /// 是否只读
    pub read_only: bool,
    /// 是否自动挂载
    pub auto_mount: bool,
    /// 挂载选项
    pub mount_options: HashMap<String, String>,
}

impl MountDirectory {
    /// 创建新的挂载目录
    pub fn new(
        name: String,
        remote_path: String,
        local_path: String,
        read_only: bool,
    ) -> Self {
        Self {
            name,
            remote_path,
            local_path,
            read_only,
            auto_mount: true,
            mount_options: HashMap::new()
        }
    }

    /// 添加挂载选项
    pub fn add_mount_option(&mut self, key: String, value: String) {
        self.mount_options.insert(key, value);
    }

    /// 获取挂载选项字符串
    pub fn mount_options_string(&self) -> String {
        self.mount_options
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(",")
    }
}

/// 文件系统连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConfig {
    /// 文件系统类型
    pub file_system_type: FileSystemType,
    /// 主机地址
    pub host: String,
    /// 端口
    pub port: u16,
    /// 用户名
    pub username: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 是否启用SSL
    pub ssl_enabled: bool,
    /// 认证令牌（用于S3等）
    pub auth_token: Option<String>,
    /// 密钥ID（用于S3等）
    pub access_key_id: Option<String>,
    /// 密钥（用于S3等）
    pub secret_access_key: Option<String>,
    /// 区域（用于S3等）
    pub region: Option<String>,
    /// 桶名称（用于S3等）
    pub bucket: Option<String>,

    /// 挂载目录列表
    pub mount_directories: Vec<MountDirectory>,
}

impl FileSystemConfig {
    /// 创建新的文件系统配置
    pub fn new(
        file_system_type: FileSystemType,
        host: String,
        port: Option<u16>,
        ssl_enabled: bool,
    ) -> Self {
        let port = port.unwrap_or_else(|| file_system_type.default_port());
        
        Self {
            file_system_type,
            host,
            port,
            ssl_enabled,
            username: None,
            password: None,
            auth_token: None,
            access_key_id: None,
            secret_access_key: None,
            region: None,
            bucket: None,
            mount_directories: Vec::new(),
        }
    }

    /// 获取连接URL
    pub fn connection_url(&self) -> String {
        match self.file_system_type {
            FileSystemType::HDFS => format!("hdfs://{}:{}", self.host, self.port),
            FileSystemType::S3 => {
                let protocol = if self.ssl_enabled { "https" } else { "http" };
                format!("{}://{}.s3.{}.amazonaws.com", protocol, self.bucket.as_deref().unwrap_or(""), self.region.as_deref().unwrap_or("us-east-1"))
            }
            FileSystemType::Local => format!("file://{}", self.host),
            FileSystemType::FTP => {
                let protocol = if self.ssl_enabled { "ftps" } else { "ftp" };
                format!("{}://{}:{}", protocol, self.host, self.port)
            }
            FileSystemType::SFTP => format!("sftp://{}:{}", self.host, self.port),
        }
    }

    /// 验证配置是否有效
    pub fn is_valid(&self) -> bool {
        !self.host.is_empty()
            && !self.host.is_empty()
            && self.port > 0
    }

    /// 添加挂载目录
    pub fn add_mount_directory(&mut self, mount_dir: MountDirectory) {
        self.mount_directories.push(mount_dir);
    }

    /// 移除挂载目录
    pub fn remove_mount_directory(&mut self, name: &str) -> bool {
        let initial_len = self.mount_directories.len();
        self.mount_directories.retain(|dir| dir.name != name);
        let removed = initial_len != self.mount_directories.len();
        removed
    }

    /// 获取挂载目录
    pub fn get_mount_directory(&self, name: &str) -> Option<&MountDirectory> {
        self.mount_directories.iter().find(|dir| dir.name == name)
    }

    /// 更新配置
    pub fn update(&mut self, updates: FileSystemConfigUpdate) {
        if let Some(host) = updates.host {
            self.host = host;
        }
        if let Some(port) = updates.port {
            self.port = port;
        }
        if let Some(username) = updates.username {
            self.username = Some(username);
        }
        if let Some(password) = updates.password {
            self.password = Some(password);
        }
        if let Some(auth_token) = updates.auth_token {
            self.auth_token = Some(auth_token);
        }
        if let Some(access_key_id) = updates.access_key_id {
            self.access_key_id = Some(access_key_id);
        }
        if let Some(secret_access_key) = updates.secret_access_key {
            self.secret_access_key = Some(secret_access_key);
        }
        if let Some(region) = updates.region {
            self.region = Some(region);
        }
        if let Some(bucket) = updates.bucket {
            self.bucket = Some(bucket);
        }


    }
}

/// 文件系统配置更新结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConfigUpdate {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub auth_token: Option<String>,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub region: Option<String>,
    pub bucket: Option<String>,
}

/// 文件系统连接测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemTestResult {
    pub success: bool,
    pub message: String,
    pub connection_time_ms: u64,
    pub available_space: Option<u64>,
    pub total_space: Option<u64>,
    pub mount_directories: Vec<String>,
}

/// 文件系统配置列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConfigList {
    pub configs: Vec<FileSystemConfig>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
}

/// 文件系统配置创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileSystemConfigRequest {
    pub name: String,
    pub file_system_type: FileSystemType,
    pub host: String,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub auth_token: Option<String>,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub region: Option<String>,
    pub bucket: Option<String>,
    pub connection_timeout: Option<u64>,
    pub read_timeout: Option<u64>,
    pub ssl_enabled: Option<bool>,
    pub extra_params: Option<HashMap<String, String>>,
}

/// 文件系统统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemStats {
    pub total_mounts: usize,
    pub active_mounts: usize,
    pub total_space_bytes: u64,
    pub used_space_bytes: u64,
    pub available_space_bytes: u64,
    pub read_operations: u64,
    pub write_operations: u64,
    pub error_count: u64,
}

impl Default for FileSystemStats {
    fn default() -> Self {
        Self {
            total_mounts: 0,
            active_mounts: 0,
            total_space_bytes: 0,
            used_space_bytes: 0,
            available_space_bytes: 0,
            read_operations: 0,
            write_operations: 0,
            error_count: 0,
        }
    }
} 