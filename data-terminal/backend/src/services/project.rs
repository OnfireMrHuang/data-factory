
use crate::repositories::{ProjectRepo};
use crate::models::{Error};
use crate::models::web::PageQuery;
use crate::models::project::{Project, CreateStatus};
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
    async fn add_project(&self, mut project: Project) -> Result<String, Error> {
        project.create_status = CreateStatus::Pending;
        project.create_msg = "".to_string();
        project.logo = "".to_string();
        let result = self.repo.add_project(project).await;
        match result {
            Ok(project) => Ok(project),
            Err(e) => Err(e),
        }
    }

    async fn edit_project(&self, project: Project) -> Result<(), Error> {
        let result = self.repo.edit_project(project).await;
        match result {
            Ok(_) => Ok(()),
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

    async fn get_project(&self, code: String) -> Result<Project, Error> {
        let result = self.repo.get_project(code).await;
        match result {
            Ok(project) => Ok(project),
            Err(e) => Err(e),
        }
    }
    async fn list_project(&self, params: PageQuery) -> Result<Vec<Project>, Error> {
        let result = self.repo.list_project(params).await;
        match result {
            Ok(projects) => Ok(projects),
            Err(e) => Err(e),
        }
    }
}


