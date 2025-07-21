use super::ProjectRepo;
use crate::models::project::Project;
use crate::models::{Validator, error::Error};
use crate::utils::database::get_config_db;
use async_trait::async_trait;
use shaku::Provider;
use sqlx::Executor;
use crate::models::web::PageQuery;

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
        let create_status = project.create_status.clone();
        let create_msg = project.create_msg.clone();
        let logo = project.logo.clone();

        let sql = "INSERT INTO df_c_project (code, name, description,create_status,create_msg, logo) VALUES (?, ?, ?, ?, ?, ?)";
        let result = pool
            .execute(
                sqlx::query(sql)
                    .bind(&code)
                    .bind(&name)
                    .bind(&description)
                    .bind(&create_status)
                    .bind(&create_msg)
                    .bind(&logo),
            )
            .await?;

        Ok(code)
    }

    async fn edit_project(&self, project: Project) -> Result<(), Error> {
        project.validate()?;

        let pool = get_config_db();
        let code = project.code.clone();
        let name = project.name.clone();
        let description = project.description.clone();
        let logo = project.logo.clone();

        let sql =
            "UPDATE df_c_project SET name = ?, description = ?, logo = ? WHERE code = ?";
        let result = pool
            .execute(
                sqlx::query(sql)
                    .bind(&name)
                    .bind(&description)
                    .bind(&logo)
                    .bind(&code),
            )
            .await?;

        Ok(())
    }

    async fn del_project(&self, code: String) -> Result<(), Error> {
        let pool = get_config_db();

        let sql = "DELETE FROM df_c_project WHERE code = ?";
        let _ = pool.execute(sqlx::query(sql).bind(&code)).await?;

        Ok(())
    }

    async fn get_project(&self, code: String) -> Result<Project, Error> {
        let pool = get_config_db();
        let sql = "SELECT * FROM df_c_project WHERE code = ?";
        let result = sqlx::query_as::<_, Project>(sql)
            .bind(&code)
            .fetch_one(&pool)
            .await?;

        Ok(result)
    }

    async fn list_project(&self, params: PageQuery) -> Result<Vec<Project>, Error> {
        let pool = get_config_db();
        let keyword = params.keyword.unwrap_or_default();
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let offset = (page - 1) * page_size;
        let sql = "SELECT * FROM df_c_project WHERE name LIKE ? LIMIT ? OFFSET ?";
        let rows = sqlx::query_as::<_, Project>(sql)
            .bind(format!("%{}%", keyword))
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&pool)
            .await?;

        Ok(rows)
    }
}