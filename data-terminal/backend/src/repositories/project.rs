use crate::models::{error::Error, Validator};
use crate::models::project::Project;
use shaku::{Provider};
use async_trait::async_trait;
use super::ProjectRepo;
use crate::utils::database::get_config_db;
use sqlx::Executor;
#[derive(Provider)]
#[shaku(interface = ProjectRepo)]
pub struct ProjectRepoImpl {}

#[async_trait]
impl ProjectRepo for ProjectRepoImpl {
    async fn add_project(&self, project: Project) -> Result<String, Error> {

        project.validate()?;

        let pool = get_config_db();
        let code = project.code.clone();
        let name = project.name.clone();
        let description = project.description.clone();
        let created_at = project.created_at.clone();
        let updated_at = project.updated_at.clone();

        let sql = "INSERT INTO projects (code, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)";
        let result = pool.execute(sqlx::query(sql).bind(&code).bind(&name).bind(&description).bind(&created_at).bind(&updated_at)).await?;
        println!("result: {:?}", result);

        Ok(code)
    }

    async fn edit_project(&self, project: Project) -> Result<(), Error> {
        project.validate()?;

        let pool = get_config_db();
        let code = project.code.clone();
        let name = project.name.clone();
        let description = project.description.clone();
        let updated_at = project.updated_at.clone();

        let sql = "UPDATE projects SET name = ?, description = ?, updated_at = ? WHERE code = ?";
        let result = pool.execute(sqlx::query(sql).bind(&name).bind(&description).bind(&updated_at).bind(&code)).await?;
        println!("result: {:?}", result);

        Ok(())
    }

    async fn del_project(&self, code: String) -> Result<(), Error> {
    let pool = get_config_db();
    
    let sql = "DELETE FROM projects WHERE code = ?";
    let result = pool.execute(sqlx::query(sql).bind(&code)).await?;
    println!("result: {:?}", result);
    
    Ok(())
    }

    async fn get_project(&self, code: String) -> Result<Project, Error> {
        let pool = get_config_db();
        let sql = "SELECT * FROM projects WHERE code = ?";
        let result = sqlx::query_as::<_, Project>(sql)
            .bind(&code)
            .fetch_one(&pool)
            .await?;
        
        Ok(result)
    }

    async fn list_project(&self) -> Result<Vec<Project>, Error> {
    let pool = get_config_db();
    let sql = "SELECT * FROM projects";
    let rows = sqlx::query_as::<_, Project>(sql)
        .fetch_all(&pool)
        .await?;
    
    Ok(rows)
    }
}
