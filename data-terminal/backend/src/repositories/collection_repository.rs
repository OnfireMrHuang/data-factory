use async_trait::async_trait;
use shaku::Interface;
use sqlx::{MySql, Pool};

use crate::models::collection::{CollectTask, CollectionCategory, CollectType, TaskStatus};

/// Collection repository trait for data access operations
#[async_trait]
pub trait CollectionRepository: Interface {
    async fn create(&self, task: &CollectTask) -> Result<CollectTask, sqlx::Error>;
    async fn find_by_id(&self, id: &str) -> Result<Option<CollectTask>, sqlx::Error>;
    async fn update(&self, task: &CollectTask) -> Result<CollectTask, sqlx::Error>;
    async fn delete(&self, id: &str) -> Result<(), sqlx::Error>;
    async fn find_all(
        &self,
        page: i64,
        limit: i64,
        status: Option<TaskStatus>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<Vec<CollectTask>, sqlx::Error>;
    async fn count_all(
        &self,
        status: Option<TaskStatus>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<i64, sqlx::Error>;
}

/// Collection repository implementation
#[derive(Clone)]
pub struct CollectionRepositoryImpl {
    pool: Pool<MySql>,
}

impl CollectionRepositoryImpl {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CollectionRepository for CollectionRepositoryImpl {
    async fn create(&self, task: &CollectTask) -> Result<CollectTask, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO df_c_collection
            (id, name, description, category, collect_type, datasource_id, resource_id, rule, status, created_at, updated_at, applied_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            task.id,
            task.name,
            task.description,
            task.category,
            task.collect_type,
            task.datasource_id,
            task.resource_id,
            serde_json::to_value(&task.rule).unwrap(),
            task.status,
            task.created_at,
            task.updated_at,
            task.applied_at
        )
        .execute(&self.pool)
        .await?;

        Ok(task.clone())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<CollectTask>, sqlx::Error> {
        let result = sqlx::query_as!(
            CollectTask,
            r#"
            SELECT
                id,
                name,
                description,
                category as "category: _",
                collect_type as "collect_type: _",
                datasource_id,
                resource_id,
                rule as "rule: _",
                status as "status: _",
                created_at,
                updated_at,
                applied_at
            FROM df_c_collection
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update(&self, task: &CollectTask) -> Result<CollectTask, sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE df_c_collection
            SET name = ?, description = ?, rule = ?, status = ?, updated_at = ?, applied_at = ?
            WHERE id = ?
            "#,
            task.name,
            task.description,
            serde_json::to_value(&task.rule).unwrap(),
            task.status,
            task.updated_at,
            task.applied_at,
            task.id
        )
        .execute(&self.pool)
        .await?;

        Ok(task.clone())
    }

    async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM df_c_collection
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_all(
        &self,
        page: i64,
        limit: i64,
        status: Option<TaskStatus>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<Vec<CollectTask>, sqlx::Error> {
        let offset = (page - 1) * limit;

        // Build dynamic query based on filters
        let mut query = String::from(
            "SELECT id, name, description, category, collect_type, datasource_id, resource_id, rule, status, created_at, updated_at, applied_at FROM df_c_collection WHERE 1=1"
        );

        if status.is_some() {
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

        if let Some(s) = status {
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
        status: Option<TaskStatus>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<i64, sqlx::Error> {
        let mut query = String::from("SELECT COUNT(*) as count FROM df_c_collection WHERE 1=1");

        if status.is_some() {
            query.push_str(" AND status = ?");
        }
        if category.is_some() {
            query.push_str(" AND category = ?");
        }
        if collect_type.is_some() {
            query.push_str(" AND collect_type = ?");
        }

        let mut query_builder = sqlx::query_scalar::<_, i64>(&query);

        if let Some(s) = status {
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

impl shaku::Component for CollectionRepositoryImpl {
    type Interface = dyn CollectionRepository;
    type Parameters = Pool<MySql>;

    fn build(
        _context: &mut shaku::ModuleBuildContext<Self>,
        params: Self::Parameters,
    ) -> Box<Self::Interface> {
        Box::new(Self::new(params))
    }
}
