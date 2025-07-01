use crate::models::{error::Error, Validator};
use crate::models::project::Project;
use shaku::{Provider};
use async_trait::async_trait;
use super::ProjectRepo;

#[derive(Provider)]
#[shaku(interface = ProjectRepo)]
pub struct ProjectRepoImpl {}

#[async_trait]
impl ProjectRepo for ProjectRepoImpl {
    async fn add_project(&self, project: Project) -> Result<String, Error> {

        project.validate()?;

        // let pool = database::get_config_db();
        // let code = project.code.clone();


        Ok("".to_string())
    }

    async fn edit_project(&self, project: Project) -> Result<(), Error> {
        Ok(())
    }

    async fn del_project(&self, code: String) -> Result<(), Error> {
        Ok(())
    }

    async fn get_project(&self, code: String) -> Result<Project, Error> {
        Err(Error::NotFound)
    }

    async fn list_project(&self) -> Result<Vec<Project>, Error> {
        Ok(Vec::new())
    }
}
