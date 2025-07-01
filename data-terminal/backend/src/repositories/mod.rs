pub mod project;



use async_trait::async_trait;
use shaku::Interface;
use crate::models::project::Project;
use crate::models::error::Error;

#[async_trait]
pub trait ProjectRepo: Interface {
    async fn add_project(&self, project: Project) -> Result<String, Error>;
    async fn edit_project(&self, project: Project) -> Result<(), Error>;
    async fn del_project(&self, code: String) -> Result<(), Error>;
    async fn get_project(&self, code: String) -> Result<Project, Error>;
    async fn list_project(&self) -> Result<Vec<Project>, Error>;
}