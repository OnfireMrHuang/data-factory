use super::ResourceRepo;
use crate::models::resource::Resource;
use crate::models::{Validator, error::Error};
use crate::utils::database::get_config_db;
use async_trait::async_trait;
use shaku::Provider;
use sqlx::Executor;
use crate::models::web::PageQuery;

#[derive(Provider)]
#[shaku(interface = ResourceRepo)]
pub struct ResourceRepoImpl {}

#[async_trait]
impl ResourceRepo for ResourceRepoImpl {
    async fn add_resource(&self, resource: Resource) -> Result<String, Error> {
        resource.validate()?;

        let pool = get_config_db().await?;
        let id = resource.id.clone();
        let name = resource.name.clone();
        let description = resource.description.clone();
        let category = resource.category;
        let resource_type = resource.resource_type;
        let config = resource.config.clone();
        let created_at = resource.created_at;
        let updated_at = resource.updated_at;
        let status = resource.status;

        let sql = "INSERT INTO df_c_resource (id, name, description, category, resource_type, config, created_at, updated_at, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let _ = pool
            .execute(
                sqlx::query(sql)
                    .bind(&id)
                    .bind(&name)
                    .bind(&description)
                    .bind(&category)
                    .bind(&resource_type)
                    .bind(&config)
                    .bind(&created_at)
                    .bind(&updated_at)
                    .bind(&status),
            )
            .await?;

        Ok(id)
    }

    async fn edit_resource(&self, resource: Resource) -> Result<(), Error> {
        resource.validate()?;

        let pool = get_config_db().await?;
        let id = resource.id.clone();
        let name = resource.name.clone();
        let description = resource.description.clone();
        let category = resource.category;
        let resource_type = resource.resource_type;
        let config = resource.config.clone();
        let updated_at = resource.updated_at;
        let status = resource.status;

        let sql = "UPDATE df_c_resource SET name = ?, description = ?, category = ?, resource_type = ?, config = ?, updated_at = ?, status = ? WHERE id = ?";
        let _ = pool
            .execute(
                sqlx::query(sql)
                    .bind(&name)
                    .bind(&description)
                    .bind(&category)
                    .bind(&resource_type)
                    .bind(&config)
                    .bind(&updated_at)
                    .bind(&status)
                    .bind(&id),
            )
            .await?;

        Ok(())
    }

    async fn del_resource(&self, id: String) -> Result<(), Error> {
        let pool = get_config_db().await?;

        let sql = "DELETE FROM df_c_resource WHERE id = ?";
        let _ = pool.execute(sqlx::query(sql).bind(&id)).await?;

        Ok(())
    }

    async fn get_resource(&self, id: String) -> Result<Resource, Error> {
        let pool = get_config_db().await?;
        let sql = "SELECT * FROM df_c_resource WHERE id = ?";
        let result = sqlx::query_as::<_, Resource>(sql)
            .bind(&id)
            .fetch_one(&pool)
            .await?;

        Ok(result)
    }

    async fn list_resource(&self, params: PageQuery) -> Result<Vec<Resource>, Error> {
        let pool = get_config_db().await?;
        let keyword = params.keyword.unwrap_or_default();
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let offset = (page - 1) * page_size;
        let sql = "SELECT * FROM df_c_resource WHERE name LIKE ? LIMIT ? OFFSET ?";
        let rows = sqlx::query_as::<_, Resource>(sql)
            .bind(format!("%{}%", keyword))
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&pool)
            .await?;

        Ok(rows)
    }
}