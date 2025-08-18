use super::DataSourceRepo;
use crate::models::datasource::DataSource;
use crate::models::{Validator, error::Error};
use crate::utils::database::get_project_db;
use async_trait::async_trait;
use shaku::Provider;
use sqlx::Executor;
use crate::models::web::PageQuery;

#[derive(Provider)]
#[shaku(interface = DataSourceRepo)]
pub struct DataSourceRepoImpl {}

#[async_trait]
impl DataSourceRepo for DataSourceRepoImpl {
    async fn add_datasource(&self, project_code: String, datasource: DataSource) -> Result<String, Error> {
        datasource.validate()?;

        let pool = get_project_db(project_code);
        let id = datasource.id.clone();
        let name = datasource.name.clone();
        let description = datasource.description.clone();
        let datasource_category = datasource.category;
        let datasource_type = datasource.datasource_type;
        let connection_config = datasource.connection_config.clone();
        let connection_status = datasource.connection_status;
        let created_at = datasource.created_at;
        let updated_at = datasource.updated_at;

        let sql = "INSERT INTO df_c_datasource (id, name, description, category, datasource_type, connection_config, connection_status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let _ = pool
            .execute(
                sqlx::query(sql)
                    .bind(&id)
                    .bind(&name)
                    .bind(&description)
                    .bind(&datasource_category)
                    .bind(&datasource_type)
                    .bind(&connection_config)
                    .bind(&connection_status)
                    .bind(&created_at)
                    .bind(&updated_at)
            )
            .await?;

        Ok(id)
    }

    async fn edit_datasource(&self, project_code: String, datasource: DataSource) -> Result<(), Error> {
        datasource.validate()?;

        let pool = get_project_db(project_code);
        let id = datasource.id.clone();
        let name = datasource.name.clone();
        let description = datasource.description.clone();
        let datasource_category = datasource.category;
        let datasource_type = datasource.datasource_type;
        let connection_config = datasource.connection_config.clone();
        let connection_status = datasource.connection_status;
        let updated_at = datasource.updated_at;

        let sql = "UPDATE df_c_datasource SET name = ?, description = ?, category = ?, datasource_type = ?, connection_config = ?, connection_status = ?, updated_at = ? WHERE id = ?";
        let _ = pool
            .execute(
                sqlx::query(sql)
                    .bind(&name)
                    .bind(&description)
                    .bind(&datasource_category)
                    .bind(&datasource_type)
                    .bind(&connection_config)
                    .bind(&connection_status)
                    .bind(&updated_at)
                    .bind(&id),
            )
            .await?;

        Ok(())
    }

    async fn del_datasource(&self, project_code: String, id: String) -> Result<(), Error> {
        let pool = get_project_db(project_code);

        let sql = "DELETE FROM df_c_datasource WHERE id = ?";
        let _ = pool.execute(sqlx::query(sql).bind(&id)).await?;

        Ok(())
    }

    async fn get_datasource(&self, project_code: String, id: String) -> Result<DataSource, Error> {
        let pool = get_project_db(project_code);
        let sql = "SELECT * FROM df_c_datasource WHERE id = ?";
        let result = sqlx::query_as::<_, DataSource>(sql)
            .bind(&id)
            .fetch_one(&pool)
            .await?;

        Ok(result)
    }

    async fn list_datasource(&self, project_code: String, params: PageQuery) -> Result<Vec<DataSource>, Error> {
        let pool = get_project_db(project_code);
        let keyword = params.keyword.unwrap_or_default();
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let offset = (page - 1) * page_size;
        let sql = "SELECT * FROM df_c_datasource WHERE name LIKE ? LIMIT ? OFFSET ?";
        let rows = sqlx::query_as::<_, DataSource>(sql)
            .bind(format!("%{}%", keyword))
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&pool)
            .await?;

        Ok(rows)
    }

    async fn list_datasource_by_project(&self, project_code: String, params: PageQuery) -> Result<Vec<DataSource>, Error> {
        let pool = get_project_db(project_code);
        let keyword = params.keyword.unwrap_or_default();
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let offset = (page - 1) * page_size;
        let sql = "SELECT * FROM df_c_datasource WHERE name LIKE ? LIMIT ? OFFSET ?";
        let rows = sqlx::query_as::<_, DataSource>(sql)
            .bind(format!("%{}%", keyword))
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&pool)
            .await?;

        Ok(rows)
    }
}
