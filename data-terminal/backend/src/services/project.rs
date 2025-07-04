
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
        let result = self.repo.add_project(project).await;
        match result {
            Ok(project) => Ok(project),
            Err(e) => Err(e),
        }
    }

    async fn del_project(&self, code: String) -> Result<(), Error> {
        let result = self.repo.del_project(code).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn list_project(&self) -> Result<Vec<Project>, Error> {
        let result = self.repo.list_project().await;
        match result {
            Ok(projects) => Ok(projects),
            Err(e) => Err(e),
        }
    }
}


