use std::fmt::format;

use crate::repositories::{DataSourceRepo};
use crate::models::{Error};
use crate::models::web::PageQuery;
use crate::models::datasource::{DataSource, DataSourceReadOnly, DataSourceCreateUpdate, DataSourceType};
use crate::models::collection::{TableMetadata, FieldMetadata};
use shaku::Provider;
use async_trait::async_trait;
use super::DataSourceService;
use chrono;
use uuid::Uuid;
use sqlx::mysql::MySqlPoolOptions;

#[derive(Provider)]
#[shaku(interface = DataSourceService)]
pub struct DataSourceServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn DataSourceRepo>,
}

#[async_trait]
impl DataSourceService for DataSourceServiceImpl {
    async fn add_datasource(&self, project_code: String, datasource: DataSourceCreateUpdate) -> Result<String, Error> {
        let mut datasource = DataSource::from(datasource);
        datasource.id = Uuid::new_v4().to_string();
        let result = self.repo.add_datasource(project_code, datasource).await;
        match result {
            Ok(id) => Ok(id),
            Err(e) => Err(e),
        }
    }

    async fn edit_datasource(&self, project_code: String, datasource: DataSourceCreateUpdate) -> Result<(), Error> {
        // 获取现有数据源以保留状态
        let existing = self.repo.get_datasource(project_code.clone(), datasource.id.clone()).await?;
        // 如果不存在则返回错误
        if existing.id.is_empty() {
            return Err(Error::NotFound);
        }
        let mut updated_datasource = DataSource::from(datasource);
        updated_datasource.connection_status = existing.connection_status;
        updated_datasource.created_at = existing.created_at;
        
        let result = self.repo.edit_datasource(project_code, updated_datasource).await;
        result.map_err(|e| Error::InternalError(format!("Failed to edit datasource: {:?}", e)))
    }

    async fn ping_datasource(&self, project_code: String, datasource: DataSourceCreateUpdate) -> Result<(), Error> {
        let datasource = DataSource::from(datasource);
        match datasource.datasource_type {
            DataSourceType::Mysql => Ok(()), // TODO: 添加 MySQL 数据源的 ping 功能
            _ => {
                return Err(Error::NotImplemented);
            }
        }
    }

    async fn del_datasource(&self, project_code: String, id: String) -> Result<(), Error> {
        let result = self.repo.del_datasource(project_code, id).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn get_datasource(&self, project_code: String, id: String) -> Result<DataSourceReadOnly, Error> {
        let result = self.repo.get_datasource(project_code, id).await;
        match result {
            Ok(datasource) => Ok(DataSourceReadOnly::from(datasource)),
            Err(e) => Err(e),
        }
    }

    async fn list_datasource(&self, project_code: String, params: PageQuery) -> Result<Vec<DataSourceReadOnly>, Error> {
        let result = self.repo.list_datasource(project_code, params).await;
        match result {
            Ok(datasources) => Ok(datasources.into_iter().map(DataSourceReadOnly::from).collect()),
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(e);
            },
        }
    }

    /// Get tables from a database datasource (MySQL/PostgreSQL)
    async fn get_datasource_tables(&self, project_code: String, datasource_id: String) -> Result<Vec<TableMetadata>, Error> {
        // Fetch datasource configuration
        let datasource = self.repo.get_datasource(project_code, datasource_id).await?;

        // Parse connection config to get database details
        let conn_config: serde_json::Value = serde_json::from_str(&datasource.connection_config.to_string())
            .map_err(|_| Error::InternalError("Invalid connection config".to_string()))?;

        let host = conn_config["host"].as_str().unwrap_or("localhost");
        let port = conn_config["port"].as_u64().unwrap_or(3306) as u16;
        let user = conn_config["user"].as_str().unwrap_or("root");
        let password = conn_config["password"].as_str().unwrap_or("");
        let database = conn_config["database"].as_str().unwrap_or("");

        // Build connection string
        let connection_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );

        // Connect to the datasource
        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&connection_url)
            .await
            .map_err(|e| Error::InternalError(format!("Failed to connect to datasource: {}", e)))?;

        // Query INFORMATION_SCHEMA for tables
        let tables = sqlx::query!(
            r#"
            SELECT
                TABLE_NAME as table_name,
                TABLE_COMMENT as table_comment
            FROM INFORMATION_SCHEMA.TABLES
            WHERE TABLE_SCHEMA = ?
            AND TABLE_TYPE = 'BASE TABLE'
            ORDER BY TABLE_NAME
            "#,
            database
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to query tables: {}", e)))?;

        // For each table, fetch its fields
        let mut result = Vec::new();
        for table in tables {
            let table_name = table.table_name;
            let fields = self.get_table_fields(String::new(), datasource.id.clone(), table_name.clone()).await?;

            result.push(TableMetadata {
                table_name,
                table_comment: table.table_comment.unwrap_or_default(),
                fields,
            });
        }

        Ok(result)
    }

    /// Get fields from a specific table in a database datasource
    async fn get_table_fields(&self, _project_code: String, datasource_id: String, table_name: String) -> Result<Vec<FieldMetadata>, Error> {
        // Fetch datasource configuration
        let datasource = self.repo.get_datasource(String::new(), datasource_id).await?;

        // Parse connection config
        let conn_config: serde_json::Value = serde_json::from_str(&datasource.connection_config.to_string())
            .map_err(|_| Error::InternalError("Invalid connection config".to_string()))?;

        let host = conn_config["host"].as_str().unwrap_or("localhost");
        let port = conn_config["port"].as_u64().unwrap_or(3306) as u16;
        let user = conn_config["user"].as_str().unwrap_or("root");
        let password = conn_config["password"].as_str().unwrap_or("");
        let database = conn_config["database"].as_str().unwrap_or("");

        let connection_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            user, password, host, port, database
        );

        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&connection_url)
            .await
            .map_err(|e| Error::InternalError(format!("Failed to connect: {}", e)))?;

        // Query INFORMATION_SCHEMA.COLUMNS
        let fields = sqlx::query!(
            r#"
            SELECT
                COLUMN_NAME as field_name,
                COLUMN_TYPE as field_type,
                IS_NULLABLE as nullable,
                COLUMN_DEFAULT as default_value,
                COLUMN_KEY as column_key
            FROM INFORMATION_SCHEMA.COLUMNS
            WHERE TABLE_SCHEMA = ?
            AND TABLE_NAME = ?
            ORDER BY ORDINAL_POSITION
            "#,
            database,
            table_name
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to query fields: {}", e)))?;

        let result = fields
            .into_iter()
            .map(|field| FieldMetadata {
                field_name: field.field_name,
                field_type: field.field_type,
                nullable: field.nullable == "YES",
                default_value: field.default_value,
                primary_key: field.column_key == "PRI",
            })
            .collect();

        Ok(result)
    }
}
