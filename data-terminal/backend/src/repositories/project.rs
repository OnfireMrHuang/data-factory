use super::ProjectRepo;
use crate::models::project::Project;
use crate::models::{Validator, error::Error};
use crate::utils::config::Setting;
use crate::utils::database::config_db_init;
use crate::utils::database::get_config_db;
use async_trait::async_trait;
use chrono::Utc;
use shaku::Provider;
use sqlx::Executor;
use std::sync::Once;
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
        let created_at = project.created_at.clone();
        let updated_at = project.updated_at.clone();

        let sql = "INSERT INTO df_c_project (code, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)";
        let result = pool
            .execute(
                sqlx::query(sql)
                    .bind(&code)
                    .bind(&name)
                    .bind(&description)
                    .bind(&created_at)
                    .bind(&updated_at),
            )
            .await?;
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

        let sql =
            "UPDATE df_c_project SET name = ?, description = ?, updated_at = ? WHERE code = ?";
        let result = pool
            .execute(
                sqlx::query(sql)
                    .bind(&name)
                    .bind(&description)
                    .bind(&updated_at)
                    .bind(&code),
            )
            .await?;
        println!("result: {:?}", result);

        Ok(())
    }

    async fn del_project(&self, code: String) -> Result<(), Error> {
        let pool = get_config_db();

        let sql = "DELETE FROM df_c_project WHERE code = ?";
        let result = pool.execute(sqlx::query(sql).bind(&code)).await?;
        println!("result: {:?}", result);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::error::Error;
    use crate::models::project::{CreateStatus, Project};
    use crate::utils::config::Setting;
    use crate::utils::database::{config_db_init, get_config_db};
    use chrono::Utc;

    async fn setup() {
        Setting::init();
        config_db_init().await;
    }

    fn sample_project() -> Project {
        Project {
            code: "test_code".to_string(),
            name: "Test Project".to_string(),
            description: "A project for testing".to_string(),
            create_status: CreateStatus::Pending,
            create_msg: "".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    async fn cleanup_test_data() {
        let pool = get_config_db();
        let _ = sqlx::query("DELETE FROM df_c_project WHERE code = 'test_code'")
            .execute(&pool)
            .await;
    }

    #[tokio::test]
    async fn test_add_and_get_project() {
        setup().await;
        cleanup_test_data().await;
        let repo = ProjectRepoImpl {};
        let project = sample_project();
        let code = repo.add_project(project.clone()).await.unwrap();
        assert_eq!(code, "test_code");
        let fetched = repo.get_project("test_code".to_string()).await.unwrap();
        assert_eq!(fetched.code, "test_code");
        assert_eq!(fetched.name, "Test Project");
        assert_eq!(fetched.description, "A project for testing");
    }

    #[tokio::test]
    async fn test_edit_project() {
        setup().await;
        cleanup_test_data().await;
        let repo = ProjectRepoImpl {};
        let mut project = sample_project();
        let _ = repo.add_project(project.clone()).await;
        project.name = "Updated Name".to_string();
        project.description = "Updated description".to_string();
        let res = repo.edit_project(project.clone()).await;
        assert!(res.is_ok());
        let fetched = repo.get_project("test_code".to_string()).await.unwrap();
        assert_eq!(fetched.name, "Updated Name");
        assert_eq!(fetched.description, "Updated description");
    }

    #[tokio::test]
    async fn test_del_project() {
        setup().await;
        cleanup_test_data().await;
        let repo = ProjectRepoImpl {};
        let project = sample_project();
        let _ = repo.add_project(project.clone()).await;
        let fetched = repo.get_project("test_code".to_string()).await.unwrap();
        assert_eq!(fetched.code, "test_code");
        let res = repo.del_project("test_code".to_string()).await;
        assert!(res.is_ok());
        let res = repo.get_project("test_code".to_string()).await;
        assert!(matches!(res, Err(Error::NotFound)));
    }

    #[tokio::test]
    async fn test_list_project() {
        setup().await;
        cleanup_test_data().await;
        let repo = ProjectRepoImpl {};
        let project = sample_project();
        let _ = repo.add_project(project.clone()).await;
        let params = PageQuery {
            keyword: None,
            page: Some(1),
            page_size: Some(10),
        };
        let list = repo.list_project(params).await.unwrap();
        assert!(list.iter().any(|p| p.code == "test_code"));
        let test_project = list.iter().find(|p| p.code == "test_code").unwrap();
        assert_eq!(test_project.name, "Test Project");
        assert_eq!(test_project.description, "A project for testing");
    }
}
