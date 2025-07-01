pub mod project;

use async_trait::async_trait;
use crate::models::project::Project;
use crate::models::error::Error;


#[async_trait]
pub trait ProjectService {
    async fn add_project(&self, project: Project) -> Result<String, Error>;
    async fn del_project(&self, code: String) -> Result<(), Error>;
    async fn list_project(&self) -> Result<Vec<Project>, Error>;
}