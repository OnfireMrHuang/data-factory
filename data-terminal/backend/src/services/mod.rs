pub mod project;
pub mod resource;

use async_trait::async_trait;
use crate::models::project::Project;
use crate::models::error::Error;
use crate::models::web::PageQuery;
use crate::models::resource::Resource;


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
    async fn add_resource(&self, resource: Resource) -> Result<String, Error>;
    async fn edit_resource(&self, resource: Resource) -> Result<(), Error>;
    async fn del_resource(&self, id: String) -> Result<(), Error>;
    async fn get_resource(&self, id: String) -> Result<Resource, Error>;
    async fn list_resource(&self, params: PageQuery) -> Result<Vec<Resource>, Error>;
}