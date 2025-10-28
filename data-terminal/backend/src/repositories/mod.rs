pub mod project;
pub mod resource;
pub mod datasource;
pub mod collection_task;

use async_trait::async_trait;
use shaku::Interface;
use crate::models::project::Project;
use crate::models::error::Error;
use crate::models::web::PageQuery;
use crate::models::resource::Resource;
use crate::models::datasource::DataSource;

#[async_trait]
pub trait ProjectRepo: Interface {
    async fn add_project(&self, project: Project) -> Result<String, Error>;
    async fn edit_project(&self, project: Project) -> Result<(), Error>;
    async fn del_project(&self, code: String) -> Result<(), Error>;
    async fn get_project(&self, code: String) -> Result<Project, Error>;
    async fn list_project(&self, params: PageQuery) -> Result<Vec<Project>, Error>;
}

#[async_trait]
pub trait ResourceRepo: Interface {
    async fn add_resource(&self, resource: Resource) -> Result<String, Error>;
    async fn edit_resource(&self, resource: Resource) -> Result<(), Error>;
    async fn del_resource(&self, id: String) -> Result<(), Error>;
    async fn get_resource(&self, id: String) -> Result<Resource, Error>;
    async fn list_resource(&self, params: PageQuery) -> Result<Vec<Resource>, Error>;
}

#[async_trait]
pub trait DataSourceRepo: Interface {
    async fn add_datasource(&self, project_code: String, datasource: DataSource) -> Result<String, Error>;
    async fn edit_datasource(&self, project_code: String, datasource: DataSource) -> Result<(), Error>;
    async fn del_datasource(&self, project_code: String, id: String) -> Result<(), Error>;
    async fn get_datasource(&self, project_code: String, id: String) -> Result<DataSource, Error>;
    async fn list_datasource(&self, project_code: String, params: PageQuery) -> Result<Vec<DataSource>, Error>;
    async fn list_datasource_by_project(&self, project_code: String, params: PageQuery) -> Result<Vec<DataSource>, Error>;
}