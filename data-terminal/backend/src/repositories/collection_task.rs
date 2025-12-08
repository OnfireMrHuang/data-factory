use async_trait::async_trait;
use shaku::Interface;
use sqlx::{query, MySql, Pool};
use shaku::Provider;
use chrono::Utc;

use crate::models::collection::{CollectTask, CollectType, CollectionCategory, TaskStage};
use crate::models::test_run::*;
use crate::models::{Validator, error::Error};
use crate::utils::database::get_project_db;
use crate::models::web::PageQuery;


/// Collection repository trait for data access operations
#[async_trait]
pub trait CollectionRepository: Interface {
    async fn create(&self, project_code: String, task: CollectTask) -> Result<String, Error>;
    async fn find_by_id(&self, project_code: String, id: &str) -> Result<Option<CollectTask>, Error>;
    async fn find_by_code(&self, project_code: String, code: &str, stage: TaskStage) -> Result<Option<CollectTask>, Error>;
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

    async fn create_test_run(&self, project_code: String, test_run: TestRun) -> Result<String, Error>;
    async fn update_test_run_status(&self, project_code: String, id: &str, status: TestRunStatus, completed_at: Option<chrono::DateTime<Utc>>, error_message: Option<String>) -> Result<(), Error>;
    async fn find_test_run_by_id(&self, project_code: String, id: &str) -> Result<Option<TestRun>, Error>;
    async fn find_latest_test_run_by_collection(&self, project_code: String, collection_code: &str) -> Result<Option<TestRun>, Error>;

    async fn create_test_step(&self, project_code: String, step: TestRunStep) -> Result<String, Error>;
    async fn update_test_step_status(&self, project_code: String, id: &str, status: TestRunStatus, completed_at: Option<chrono::DateTime<Utc>>, error_message: Option<String>) -> Result<(), Error>;
    async fn find_test_steps_by_run_id(&self, project_code: String, test_run_id: &str) -> Result<Vec<TestRunStep>, Error>;

    async fn create_test_log(&self, project_code: String, log: TestRunLog) -> Result<i64, Error>;
    async fn find_test_logs(&self, project_code: String, test_run_id: &str, start_timestamp: Option<i64>) -> Result<Vec<TestRunLog>, Error>;
    async fn count_test_logs(&self, project_code: String, test_run_id: &str, start_timestamp: Option<i64>) -> Result<i64, Error>;
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
        let queue_resource_id = task.queue_resource_id.clone();
        let database_resource_id = task.database_resource_id.clone();
        let rule = task.rule.clone();
        let stage = task.stage;
        let created_at = task.created_at;
        let updated_at = task.updated_at;
        let applied_at = task.applied_at;

        sqlx::query(
            r#"
            INSERT INTO df_c_collection
            (id, code, name, description, category, collect_type, datasource_id, queue_resource_id, database_resource_id, rule, stage, created_at, updated_at, applied_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&code)
        .bind(&name)
        .bind(&description)
        .bind(&category)
        .bind(&collect_type)
        .bind(&datasource_id)
        .bind(&queue_resource_id)
        .bind(&database_resource_id)
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
                queue_resource_id,
                database_resource_id,
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

    async fn find_by_code(&self, project_code: String, code: &str, stage: TaskStage) -> Result<Option<CollectTask>, Error> {
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
                queue_resource_id,
                database_resource_id,
                rule,
                stage,
                created_at,
                updated_at,
                applied_at
            FROM df_c_collection
            WHERE code = ? AND stage = ?
            "#
        )
        .bind(code)
        .bind(stage)
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
            "SELECT id, code, name, description, category, collect_type, datasource_id, queue_resource_id, database_resource_id, rule, stage, created_at, updated_at, applied_at FROM df_c_collection WHERE 1=1"
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

    async fn create_test_run(&self, project_code: String, test_run: TestRun) -> Result<String, Error> {
        let pool = get_project_db(project_code).await?;
        let id = test_run.id.clone();

        sqlx::query(
            r#"
            INSERT INTO df_c_collection_test_run
            (id, collection_code, status, started_at, completed_at, error_message, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&test_run.id)
        .bind(&test_run.collection_code)
        .bind(&test_run.status)
        .bind(&test_run.started_at)
        .bind(&test_run.completed_at)
        .bind(&test_run.error_message)
        .bind(&test_run.created_at)
        .bind(&test_run.updated_at)
        .execute(&pool)
        .await?;

        Ok(id)
    }

    async fn update_test_run_status(
        &self,
        project_code: String,
        id: &str,
        status: TestRunStatus,
        completed_at: Option<chrono::DateTime<Utc>>,
        error_message: Option<String>,
    ) -> Result<(), Error> {
        let pool = get_project_db(project_code).await?;

        sqlx::query(
            r#"
            UPDATE df_c_collection_test_run
            SET status = ?, completed_at = ?, error_message = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&status)
        .bind(&completed_at)
        .bind(&error_message)
        .bind(Utc::now())
        .bind(id)
        .execute(&pool)
        .await?;

        Ok(())
    }

    async fn find_test_run_by_id(&self, project_code: String, id: &str) -> Result<Option<TestRun>, Error> {
        let pool = get_project_db(project_code).await?;

        let result = sqlx::query_as::<_, TestRun>(
            r#"
            SELECT id, collection_code, status, started_at, completed_at, error_message, created_at, updated_at
            FROM df_c_collection_test_run
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&pool)
        .await?;

        Ok(result)
    }

    async fn find_latest_test_run_by_collection(&self, project_code: String, collection_code: &str) -> Result<Option<TestRun>, Error> {
        let pool = get_project_db(project_code).await?;

        let result = sqlx::query_as::<_, TestRun>(
            r#"
            SELECT id, collection_code, status, started_at, completed_at, error_message, created_at, updated_at
            FROM df_c_collection_test_run
            WHERE collection_code = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(collection_code)
        .fetch_optional(&pool)
        .await?;

        Ok(result)
    }

    async fn create_test_step(&self, project_code: String, step: TestRunStep) -> Result<String, Error> {
        let pool = get_project_db(project_code).await?;
        let id = step.id.clone();

        sqlx::query(
            r#"
            INSERT INTO df_c_collection_test_run_step
            (id, test_run_id, step_id, title, description, status, started_at, completed_at, error_message, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&step.id)
        .bind(&step.test_run_id)
        .bind(&step.step_id)
        .bind(&step.title)
        .bind(&step.description)
        .bind(&step.status)
        .bind(&step.started_at)
        .bind(&step.completed_at)
        .bind(&step.error_message)
        .bind(&step.created_at)
        .bind(&step.updated_at)
        .execute(&pool)
        .await?;

        Ok(id)
    }

    async fn update_test_step_status(
        &self,
        project_code: String,
        id: &str,
        status: TestRunStatus,
        completed_at: Option<chrono::DateTime<Utc>>,
        error_message: Option<String>,
    ) -> Result<(), Error> {
        let pool = get_project_db(project_code).await?;

        sqlx::query(
            r#"
            UPDATE df_c_collection_test_run_step
            SET status = ?, completed_at = ?, error_message = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&status)
        .bind(&completed_at)
        .bind(&error_message)
        .bind(Utc::now())
        .bind(id)
        .execute(&pool)
        .await?;

        Ok(())
    }

    async fn find_test_steps_by_run_id(&self, project_code: String, test_run_id: &str) -> Result<Vec<TestRunStep>, Error> {
        let pool = get_project_db(project_code).await?;

        let results = sqlx::query_as::<_, TestRunStep>(
            r#"
            SELECT id, test_run_id, step_id, title, description, status, started_at, completed_at, error_message, created_at, updated_at
            FROM df_c_collection_test_run_step
            WHERE test_run_id = ?
            ORDER BY step_id ASC
            "#
        )
        .bind(test_run_id)
        .fetch_all(&pool)
        .await?;

        Ok(results)
    }

    async fn create_test_log(&self, project_code: String, log: TestRunLog) -> Result<i64, Error> {
        let pool = get_project_db(project_code).await?;

        let result = sqlx::query(
            r#"
            INSERT INTO df_c_collection_test_run_log
            (created_at, test_run_id, log_level, message, details)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(&log.created_at)
        .bind(&log.test_run_id)
        .bind(&log.log_level)
        .bind(&log.message)
        .bind(&log.details)
        .execute(&pool)
        .await?;

        Ok(result.last_insert_id() as i64)
    }

    async fn find_test_logs(
        &self,
        project_code: String,
        test_run_id: &str,
        start_timestamp: Option<i64>,
    ) -> Result<Vec<TestRunLog>, Error> {
        let pool = get_project_db(project_code).await?;

        let results = if let Some(timestamp) = start_timestamp {
            sqlx::query_as::<_, TestRunLog>(
                r#"
                SELECT id, created_at, test_run_id, log_level, message, details
                FROM df_c_collection_test_run_log
                WHERE test_run_id = ? AND created_at > ?
                ORDER BY created_at ASC
                LIMIT 1000
                "#
            )
            .bind(test_run_id)
            .bind(timestamp)
            .fetch_all(&pool)
            .await?
        } else {
            sqlx::query_as::<_, TestRunLog>(
                r#"
                SELECT id, created_at, test_run_id, log_level, message, details
                FROM df_c_collection_test_run_log
                WHERE test_run_id = ?
                ORDER BY created_at ASC
                LIMIT 1000
                "#
            )
            .bind(test_run_id)
            .fetch_all(&pool)
            .await?
        };

        Ok(results)
    }

    async fn count_test_logs(
        &self,
        project_code: String,
        test_run_id: &str,
        start_timestamp: Option<i64>,
    ) -> Result<i64, Error> {
        let pool = get_project_db(project_code).await?;

        let count = if let Some(timestamp) = start_timestamp {
            sqlx::query_scalar::<_, i64>(
                r#"
                SELECT COUNT(*) as count
                FROM df_c_collection_test_run_log
                WHERE test_run_id = ? AND created_at > ?
                "#
            )
            .bind(test_run_id)
            .bind(timestamp)
            .fetch_one(&pool)
            .await?
        } else {
            sqlx::query_scalar::<_, i64>(
                r#"
                SELECT COUNT(*) as count
                FROM df_c_collection_test_run_log
                WHERE test_run_id = ?
                "#
            )
            .bind(test_run_id)
            .fetch_one(&pool)
            .await?
        };

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