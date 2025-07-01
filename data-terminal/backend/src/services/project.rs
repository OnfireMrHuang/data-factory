
use crate::repositories::{ProjectRepo};
use crate::models::{Error};
use crate::models::project::Project;
use shaku::Provider;
use async_trait::async_trait;
use super::ProjectService;


#[derive(Provider)]
#[shaku(interface = ProjectService)]
pub struct ProjectServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn ProjectRepo>,
}


#[async_trait]
impl ProjectService for ProjectServiceImpl {
    async fn add_project(&self, project: Project) -> Result<String, Error> {
        Err(Error::NotImplemented)
    }

    async fn del_project(&self, code: String) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }

    async fn list_project(&self) -> Result<Vec<Project>, Error> {
        Err(Error::NotImplemented)
    }
}


