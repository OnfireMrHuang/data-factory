use async_trait::async_trait;
use shaku::Interface;
use sqlx::MySql;
use shaku::Provider;
use chrono::Utc;

use crate::models::test_run::*;
use crate::models::error::Error;
use crate::utils::database::get_project_db;

/// Test run repository trait for data access operations
#[async_trait]
pub trait TestRunRepository: Interface {
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

/// Test run repository implementation
#[derive(Provider)]
#[shaku(interface = TestRunRepository)]
pub struct TestRunRepositoryImpl {}

#[async_trait]
impl TestRunRepository for TestRunRepositoryImpl {
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
