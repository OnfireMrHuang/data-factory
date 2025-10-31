use async_trait::async_trait;
use shaku::Interface;
use sqlx::{query, MySql, Pool};
use shaku::Provider;

use crate::models::collection::{CollectTask, CollectType, CollectionCategory, TaskStage};
use crate::models::{Validator, error::Error};
use crate::utils::database::get_project_db;
use crate::models::web::PageQuery;

/// Collection repository trait for data access operations
#[async_trait]
pub trait CollectionRepository: Interface {
    async fn create(&self, project_code: String, task: CollectTask) -> Result<String, Error>;
    async fn find_by_id(&self, project_code: String, id: &str) -> Result<Option<CollectTask>, Error>;
    async fn update(&self, project_code: String, task: CollectTask) -> Result<(), Error>;
    async fn delete_task(&self, project_code: String, code: &str) -> Result<(), Error>;
    async fn delete_by_code(&self, project_code: String, code: &str, stage: TaskStage) -> Result<(), Error>;
    async fn find_all(
        &self,
        project_code: String,
        params: PageQuery,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<Vec<CollectTask>, Error>;
    async fn count_all(
        &self,
        project_code: String,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<i64, Error>;
}

/// Collection repository implementation
#[derive(Provider)]
#[shaku(interface = CollectionRepository)]
pub struct CollectionRepositoryImpl {}

#[async_trait]
impl CollectionRepository for CollectionRepositoryImpl {
    async fn create(&self, project_code: String, task: CollectTask) -> Result<String, Error> {
        task.validate()?;

        let pool = get_project_db(project_code).await?;
        let id = task.id.clone();
        let code = task.code.clone();
        let name = task.name.clone();
        let description = task.description.clone();
        let category = task.category;
        let collect_type = task.collect_type;
        let datasource_id = task.datasource_id.clone();
        let resource_id = task.resource_id.clone();
        let rule = task.rule.clone();
        let stage = task.stage;
        let created_at = task.created_at;
        let updated_at = task.updated_at;
        let applied_at = task.applied_at;

        sqlx::query(
            r#"
            INSERT INTO df_c_collection
            (id, code, name, description, category, collect_type, datasource_id, resource_id, rule, stage, created_at, updated_at, applied_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&code)
        .bind(&name)
        .bind(&description)
        .bind(&category)
        .bind(&collect_type)
        .bind(&datasource_id)
        .bind(&resource_id)
        .bind(serde_json::to_value(&rule).unwrap())
        .bind(&stage)
        .bind(&created_at)
        .bind(&updated_at)
        .bind(&applied_at)
        .execute(&pool)
        .await?;

        Ok(id)
    }

    async fn find_by_id(&self, project_code: String, id: &str) -> Result<Option<CollectTask>, Error> {
        let pool = get_project_db(project_code).await?;
        let result = sqlx::query_as::<_, CollectTask>(
            r#"
            SELECT
                id,
                code,
                name,
                description,
                category,
                collect_type,
                datasource_id,
                resource_id,
                rule,
                stage,
                created_at,
                updated_at,
                applied_at
            FROM df_c_collection
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, project_code: String, task: CollectTask) -> Result<(), Error> {
        task.validate()?;

        let pool = get_project_db(project_code).await?;
        let id = task.id.clone();
        let name = task.name.clone();
        let description = task.description.clone();
        let rule = task.rule.clone();
        let stage = task.stage;
        let updated_at = task.updated_at;
        let applied_at = task.applied_at;

        sqlx::query(
            r#"
            UPDATE df_c_collection
            SET name = ?, description = ?, rule = ?, stage = ?, updated_at = ?, applied_at = ?
            WHERE id = ?
            "#
        )
        .bind(&name)
        .bind(&description)
        .bind(serde_json::to_value(&rule).unwrap())
        .bind(&stage)
        .bind(&updated_at)
        .bind(&applied_at)
        .bind(&id)
        .execute(&pool)
        .await?;

        Ok(())
    }

    async fn delete_task(&self, project_code: String, code: &str) -> Result<(), Error> {
        let pool = get_project_db(project_code).await?;
        sqlx::query(
         r#"
             DELETE FROM df_c_collection
             WHERE code = ?
         "#
        ).bind(code).execute(&pool).await?;

        Ok(())
    }

    async fn delete_by_code(&self, project_code: String, code: &str, stage: TaskStage) -> Result<(), Error> {
        let pool = get_project_db(project_code).await?;
        sqlx::query(
            r#"
            DELETE FROM df_c_collection
            WHERE id = ? AND stage = ?
            "#
        ).bind(code).bind(stage)
        .execute(&pool)
        .await?;

        Ok(())
    }

    async fn find_all(
        &self,
        project_code: String,
        params: PageQuery,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<Vec<CollectTask>, Error> {
        let pool = get_project_db(project_code).await?;
        let page = params.page.unwrap_or(1) as i64;
        let page_size = params.page_size.unwrap_or(10) as i64;
        let offset = (page - 1) * page_size;
        let keyword = params.keyword.unwrap_or_default();

        // Build dynamic query based on filters
        let mut query = String::from(
            "SELECT id, code, name, description, category, collect_type, datasource_id, resource_id, rule, stage, created_at, updated_at, applied_at FROM df_c_collection WHERE 1=1"
        );

        if !keyword.is_empty() {
            query.push_str(" AND name LIKE ?");
        }
        if stage.is_some() {
            query.push_str(" AND stage = ?");
        }
        if category.is_some() {
            query.push_str(" AND category = ?");
        }
        if collect_type.is_some() {
            query.push_str(" AND collect_type = ?");
        }

        query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

        // Use query builder for dynamic parameters
        let mut query_builder = sqlx::query_as::<_, CollectTask>(&query);
        
        if !keyword.is_empty() {
            query_builder = query_builder.bind(format!("%{}%", keyword));
        }
        if let Some(s) = stage {
            query_builder = query_builder.bind(s);
        }
        if let Some(c) = category {
            query_builder = query_builder.bind(c);
        }
        if let Some(ct) = collect_type {
            query_builder = query_builder.bind(ct);
        }

        query_builder = query_builder.bind(page_size).bind(offset);

        let results = query_builder.fetch_all(&pool).await?;

        Ok(results)
    }

    async fn count_all(
        &self,
        project_code: String,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<i64, Error> {
        let pool = get_project_db(project_code).await?;
        let mut query = String::from("SELECT COUNT(*) as count FROM df_c_collection WHERE 1=1");

        if stage.is_some() {
            query.push_str(" AND stage = ?");
        }
        if category.is_some() {
            query.push_str(" AND category = ?");
        }
        if collect_type.is_some() {
            query.push_str(" AND collect_type = ?");
        }

        let mut query_builder = sqlx::query_scalar::<_, i64>(&query);

        if let Some(s) = stage {
            query_builder = query_builder.bind(s);
        }
        if let Some(c) = category {
            query_builder = query_builder.bind(c);
        }
        if let Some(ct) = collect_type {
            query_builder = query_builder.bind(ct);
        }

        let count = query_builder.fetch_one(&pool).await?;

        Ok(count)
    }
}

// impl<M: shaku::Module> shaku::Component<M> for CollectionRepositoryImpl {
//     type Interface = dyn CollectionRepository;
//     type Parameters = ();

//     fn build(
//         _context: &mut shaku::ModuleBuildContext<M>,
//         _params: Self::Parameters,
//     ) -> Box<Self::Interface> {
//         Box::new(Self {})
//     }
// }