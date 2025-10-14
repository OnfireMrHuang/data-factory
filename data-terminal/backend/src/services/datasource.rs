use crate::repositories::{DataSourceRepo};
use crate::models::{Error};
use crate::models::web::PageQuery;
use crate::models::datasource::{DataSource, DataSourceReadOnly, DataSourceCreateUpdate, DataSourceType};
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
        let mut updated_datasource = DataSource::from(datasource);
        updated_datasource.connection_status = existing.connection_status;
        updated_datasource.created_at = existing.created_at;
        updated_datasource.updated_at = chrono::Utc::now();
        
        let result = self.repo.edit_datasource(project_code, updated_datasource).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
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
            Err(e) => Err(e),
        }
    }
}
