use async_trait::async_trait;
use shaku::Interface;
use sqlx::{query, MySql, Pool};

use crate::models::collection::{CollectTask, CollectType, CollectionCategory, TaskStage};

/// Collection repository trait for data access operations
#[async_trait]
pub trait CollectionRepository: Interface {
    async fn create(&self, task: &CollectTask) -> Result<CollectTask, sqlx::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<CollectTask>, sqlx::Error>;
    async fn update(&self, task: &CollectTask) -> Result<CollectTask, sqlx::Error>;
    async fn delete_task(&self, code: &str) -> Result<(), sqlx::Error>;
    async fn delete_by_code(&self, code: &str, stage: TaskStage) -> Result<(), sqlx::Error>;
    async fn find_all(
        &self,
        page: i64,
        limit: i64,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<Vec<CollectTask>, sqlx::Error>;
    async fn count_all(
        &self,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<i64, sqlx::Error>;
}

/// Collection repository implementation
#[derive(Clone)]
pub struct CollectionRepositoryImpl {
    pool: Pool<MySql>,
}

impl<M: shaku::Module> shaku::Component<M> for CollectionRepositoryImpl {
    type Interface = dyn CollectionRepository;
    type Parameters = ();

    fn build(
        _context: &mut shaku::ModuleBuildContext<M>,
        _params: Self::Parameters,
    ) -> Box<Self::Interface> {
        panic!("CollectionRepositoryImpl must be initialized with a Pool<MySql> parameter. Use module.resolve_with_args instead.");
    }
}

impl CollectionRepositoryImpl {
    pub fn new_with_pool(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CollectionRepository for CollectionRepositoryImpl {
    async fn create(&self, task: &CollectTask) -> Result<CollectTask, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO df_c_collection
            (id, name, description, category, collect_type, datasource_id, resource_id, rule, stage, created_at, updated_at, applied_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&task.id)
        .bind(&task.name)
        .bind(&task.description)
        .bind(&task.category)
        .bind(&task.collect_type)
        .bind(&task.datasource_id)
        .bind(&task.resource_id)
        .bind(serde_json::to_value(&task.rule).unwrap())
        .bind(&task.stage)
        .bind(&task.created_at)
        .bind(&task.updated_at)
        .bind(&task.applied_at)
        .execute(&self.pool)
        .await?;

        Ok(task.clone())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<CollectTask>, sqlx::Error> {
        let result = sqlx::query_as::<_, CollectTask>(
            r#"
            SELECT
                id,
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
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, task: &CollectTask) -> Result<CollectTask, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE df_c_collection
            SET name = ?, description = ?, rule = ?, stage = ?, updated_at = ?, applied_at = ?
            WHERE id = ?
            "#
        )
        .bind(&task.name)
        .bind(&task.description)
        .bind(serde_json::to_value(&task.rule).unwrap())
        .bind(&task.stage)
        .bind(&task.updated_at)
        .bind(&task.applied_at)
        .bind(&task.id)
        .execute(&self.pool)
        .await?;

        Ok(task.clone())
    }

    async fn delete_task(&self, code: &str) -> Result<(), sqlx::Error> {
       sqlx::query(
        r#"
            DELETE FROM df_c_collection
            WHERE code = ?
        "#
       ).bind(code).execute(&self.pool).await?;

        Ok(())
    }

    async fn delete_by_code(&self, code: &str, stage: TaskStage) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM df_c_collection
            WHERE id = ? AND stage = ?
            "#
        ).bind(code).bind(stage)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_all(
        &self,
        page: i64,
        limit: i64,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<Vec<CollectTask>, sqlx::Error> {
        let offset = (page - 1) * limit;

        // Build dynamic query based on filters
        let mut query = String::from(
            "SELECT id, name, description, category, collect_type, datasource_id, resource_id, rule, status, created_at, updated_at, applied_at FROM df_c_collection WHERE 1=1"
        );

        if stage.is_some() {
            query.push_str(" AND status = ?");
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

        if let Some(s) = stage {
            query_builder = query_builder.bind(s);
        }
        if let Some(c) = category {
            query_builder = query_builder.bind(c);
        }
        if let Some(ct) = collect_type {
            query_builder = query_builder.bind(ct);
        }

        query_builder = query_builder.bind(limit).bind(offset);

        let results = query_builder.fetch_all(&self.pool).await?;

        Ok(results)
    }

    async fn count_all(
        &self,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<i64, sqlx::Error> {
        let mut query = String::from("SELECT COUNT(*) as count FROM df_c_collection WHERE 1=1");

        if stage.is_some() {
            query.push_str(" AND status = ?");
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

        let count = query_builder.fetch_one(&self.pool).await?;

        Ok(count)
    }
}

