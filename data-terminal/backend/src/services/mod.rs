pub mod project;
pub mod resource;
pub mod datasource;

use async_trait::async_trait;
use crate::models::project::Project;
use crate::models::error::Error;
use crate::models::web::PageQuery;
use crate::models::resource::{Resource, ResourceReadOnly, ResourceCreateUpdate};
use crate::models::datasource::{DataSource, DataSourceReadOnly, DataSourceCreateUpdate};

#[async_trait]
pub trait ProjectService: Send {
    async fn add_project(&self, project: Project) -> Result<String, Error>;
    async fn edit_project(&self, project: Project) -> Result<(), Error>;
    async fn del_project(&self, code: String) -> Result<(), Error>;
    async fn get_project(&self, code: String) -> Result<Project, Error>;
    async fn list_project(&self, params: PageQuery) -> Result<Vec<Project>, Error>;
}

#[async_trait]
pub trait ResourceService: Send {
    async fn add_resource(&self, resource: ResourceCreateUpdate) -> Result<String, Error>;
    async fn edit_resource(&self, resource: ResourceCreateUpdate) -> Result<(), Error>;
    async fn del_resource(&self, id: String) -> Result<(), Error>;
    async fn get_resource(&self, id: String) -> Result<ResourceReadOnly, Error>;
    async fn list_resource(&self, params: PageQuery) -> Result<Vec<ResourceReadOnly>, Error>;
}

#[async_trait]
pub trait DataSourceService: Send {
    async fn add_datasource(&self, project_code: String, datasource: DataSourceCreateUpdate) -> Result<String, Error>;
    async fn edit_datasource(&self, project_code: String, datasource: DataSourceCreateUpdate) -> Result<(), Error>;
    async fn del_datasource(&self, project_code: String, id: String) -> Result<(), Error>;
    async fn get_datasource(&self, project_code: String, id: String) -> Result<DataSourceReadOnly, Error>;
    async fn list_datasource(&self, project_code: String, params: PageQuery) -> Result<Vec<DataSourceReadOnly>, Error>;
}