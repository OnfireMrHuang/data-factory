use async_trait::async_trait;
use chrono::Utc;
use shaku::Interface;
use uuid::Uuid;
use shaku::Provider;

use crate::models::collection::*;
use crate::repositories::collection_task::CollectionRepository;
use crate::models::Error;

/// Collection service trait for business logic
#[async_trait]
pub trait CollectionService: Interface {
    async fn create_task(
        &self,
        project_code: String,
        request: CreateCollectTaskRequest,
    ) -> Result<CollectTaskReadOnly, Error>;

    async fn get_task(&self, project_code: String, id: &str) -> Result<Option<CollectTaskReadOnly>, Error>;

    async fn update_task(
        &self,
        project_code: String,
        request: UpdateCollectTaskRequest,
    ) -> Result<CollectTaskReadOnly, Error>;

    async fn delete_task(&self, project_code: String, id: &str) -> Result<(), Error>;

    async fn list_tasks(
        &self,
        project_code: String,
        page: i64,
        limit: i64,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<(Vec<CollectTaskReadOnly>, i64), Error>;

    async fn apply_task(&self, project_code: String, id: &str) -> Result<CollectTaskReadOnly, Error>;

    async fn generate_schema(
        &self,
        project_code: String,
        datasource_id: &str,
        resource_id: &str,
        selected_tables: Vec<TableSelection>,
    ) -> Result<TableSchema, Error>;

    async fn validate_task_config(&self, project_code: String, task: &CollectTask) -> Result<(), Error>;
}

/// Collection service implementation
#[derive(Provider)]
#[shaku(interface = CollectionService)]
pub struct CollectionServiceImpl {
    #[shaku(provide)]
    repository: Box<dyn CollectionRepository>
}

impl CollectionServiceImpl {

    /// Validate datasource/resource compatibility based on collection mode
    fn validate_compatibility(
        &self,
        category: &CollectionCategory,
        collect_type: &CollectType,
        rule: &CollectionRule,
    ) -> Result<(), Error> {
        // Validation rules from spec:
        // - Full + Database → relational_database only, must have FullDatabaseRule
        // - Full + API → relational_database or file_system, must have FullApiRule
        // - Incremental + Database → queue only, must have IncrementalDatabaseRule
        // - Incremental + API → queue only, must have IncrementalApiRule

        match (category, collect_type) {
            (CollectionCategory::Database, CollectType::Full) => {
                if !matches!(rule, CollectionRule::FullDatabase(_)) {
                    return Err(Error::InvalidValue(
                        "Full Database collection requires FullDatabaseRule".to_string()
                    ));
                }
                // Validate rule contents
                if let CollectionRule::FullDatabase(db_rule) = rule {
                    if db_rule.selected_tables.is_empty() {
                        return Err(Error::InvalidValue(
                            "Must select at least one table".to_string()
                        ));
                    }
                }
                Ok(())
            }
            (CollectionCategory::Api, CollectType::Full) => {
                if !matches!(rule, CollectionRule::FullApi(_)) {
                    return Err(Error::InvalidValue(
                        "Full API collection requires FullApiRule".to_string()
                    ));
                }
                Ok(())
            }
            (CollectionCategory::Database, CollectType::Incremental) => {
                if !matches!(rule, CollectionRule::IncrementalDatabase(_)) {
                    return Err(Error::InvalidValue(
                        "Incremental Database collection requires IncrementalDatabaseRule".to_string()
                    ));
                }
                Ok(())
            }
            (CollectionCategory::Api, CollectType::Incremental) => {
                if !matches!(rule, CollectionRule::IncrementalApi(_)) {
                    return Err(Error::InvalidValue(
                        "Incremental API collection requires IncrementalApiRule".to_string()
                    ));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl CollectionService for CollectionServiceImpl {
    async fn create_task(
        &self,
        project_code: String,
        request: CreateCollectTaskRequest,
    ) -> Result<CollectTaskReadOnly, Error> {
        // Generate UUID for new task
        let id = Uuid::new_v4().to_string();
        let code = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Validate compatibility with comprehensive rule checking
        self.validate_compatibility(&request.category, &request.collect_type, &request.rule)?;

        let task = CollectTask {
            id,
            code,
            name: request.name,
            description: request.description.unwrap_or_default(),
            category: request.category,
            collect_type: request.collect_type,
            datasource_id: request.datasource_id,
            resource_id: request.resource_id,
            rule: request.rule,
            stage: TaskStage::Draft,
            created_at: now,
            updated_at: now,
            applied_at: None,
        };

        let created_id = self.repository.create(project_code, task.clone()).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to create task: {:?}", e)),
            })?;

        // Return the created task as ReadOnly
        Ok(CollectTaskReadOnly::from(task))
    }

    async fn get_task(&self, project_code: String, id: &str) -> Result<Option<CollectTaskReadOnly>, Error> {
        let task = self.repository.find_by_id(project_code, id).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to get task: {:?}", e)),
            })?;

        Ok(task.map(CollectTaskReadOnly::from))
    }

    async fn update_task(
        &self,
        project_code: String,
        request: UpdateCollectTaskRequest,
    ) -> Result<CollectTaskReadOnly, Error> {
        // Fetch existing task
        let mut task = self.repository.find_by_id(project_code.clone(), &request.id).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to find task: {:?}", e)),
            })?
            .ok_or(Error::NotFound)?;

        // Only allow updates for Draft
        if !matches!(task.stage, TaskStage::Draft) {
            return Err(Error::InvalidOperation(
                "Cannot update task that is applied or running".to_string()
            ));
        }

        // Apply updates
        if let Some(name) = request.name {
            task.name = name;
        }
        if let Some(description) = request.description {
            task.description = description;
        }
        if let Some(rule) = request.rule {
            task.rule = rule;
        }

        task.updated_at = Utc::now();

        self.repository.update(project_code.clone(), task.clone()).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to update task: {:?}", e)),
            })?;

        Ok(CollectTaskReadOnly::from(task))
    }

    async fn delete_task(&self, project_code: String, id: &str) -> Result<(), Error> {
        // Fetch task to check status
        let task = self.repository.find_by_id(project_code.clone(), id).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to find task: {:?}", e)),
            })?
            .ok_or(Error::NotFound)?;

        // delete draft task and applied task
        self.repository.delete_task(project_code, &task.code).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to delete task: {:?}", e)),
            })
    }

    async fn list_tasks(
        &self,
        project_code: String,
        page: i64,
        limit: i64,
        stage: Option<TaskStage>,
        category: Option<CollectionCategory>,
        collect_type: Option<CollectType>,
    ) -> Result<(Vec<CollectTaskReadOnly>, i64), Error> {
        use crate::models::web::PageQuery;

        let params = PageQuery {
            keyword: None,
            page: Some(page as u64),
            page_size: Some(limit as u64),
        };

        let tasks = self.repository.find_all(project_code.clone(), params, stage.clone(), category.clone(), collect_type.clone()).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to list tasks: {:?}", e)),
            })?;

        let total = self.repository.count_all(project_code, stage, category, collect_type).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to count tasks: {:?}", e)),
            })?;

        Ok((tasks.into_iter().map(CollectTaskReadOnly::from).collect(), total))
    }

    async fn apply_task(&self, project_code: String, id: &str) -> Result<CollectTaskReadOnly, Error> {
        // Fetch task
        let mut task = self.repository.find_by_id(project_code.clone(), id).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to find task: {:?}", e)),
            })?
            .ok_or(Error::NotFound)?;

        // attampt to delete existing applied task
        self.repository.delete_by_code(project_code.clone(), &task.code, task.stage).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to delete existing task: {:?}", e)),
            })?;

        // create applied task
        task.id = Uuid::new_v4().to_string(); // recreate id
        task.stage = TaskStage::Applied; //
        task.applied_at = Some(Utc::now());
        task.updated_at = Utc::now();

        self.repository.create(project_code, task.clone()).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to apply task: {:?}", e)),
            })?;

        Ok(CollectTaskReadOnly::from(task))
    }

    async fn generate_schema(
        &self,
        _project_code: String,
        _datasource_id: &str,
        _resource_id: &str,
        selected_tables: Vec<TableSelection>,
    ) -> Result<TableSchema, Error> {
        // Validate input
        if selected_tables.is_empty() {
            return Err(Error::InvalidValue("No tables selected".to_string()));
        }

        let first_table = &selected_tables[0];

        // Type mapping logic: MySQL → Target database types
        // For now, use the first selected table as basis
        // TODO: In production, fetch actual source schema and apply type mapping rules

        // Generate target table name (prefix convention)
        let target_table_name = if first_table.table_name.starts_with("df_") {
            first_table.table_name.clone()
        } else {
            format!("target_{}", first_table.table_name)
        };

        // Mock field generation - in production, this would:
        // 1. Fetch source table schema
        // 2. Map MySQL types to target types (INT→INT, VARCHAR→VARCHAR, etc.)
        // 3. Preserve nullable, default values, primary keys
        // 4. Handle selected_fields filtering

        let fields = if first_table.selected_fields.is_empty() {
            // All fields - generate basic schema
            vec![
                FieldSchema {
                    field_name: "id".to_string(),
                    field_type: "BIGINT".to_string(),
                    nullable: false,
                    default_value: None,
                    primary_key: true,
                    auto_increment: true,
                },
                FieldSchema {
                    field_name: "created_at".to_string(),
                    field_type: "TIMESTAMP".to_string(),
                    nullable: false,
                    default_value: Some("CURRENT_TIMESTAMP".to_string()),
                    primary_key: false,
                    auto_increment: false,
                },
            ]
        } else {
            // Selected fields only - map each field
            first_table.selected_fields.iter().map(|field_name| {
                FieldSchema {
                    field_name: field_name.clone(),
                    field_type: Self::map_field_type(field_name),
                    nullable: true,
                    default_value: None,
                    primary_key: field_name == "id",
                    auto_increment: field_name == "id",
                }
            }).collect()
        };

        Ok(TableSchema {
            table_name: target_table_name,
            fields,
        })
    }

    async fn validate_task_config(&self, _project_code: String, _task: &CollectTask) -> Result<(), Error> {
        // TODO: Implement comprehensive validation (US2-US4)
        Ok(())
    }
}

impl CollectionServiceImpl {
    /// Map source field types to target types
    /// This is a simplified mapping - production would have comprehensive rules
    fn map_field_type(field_name: &str) -> String {
        match field_name {
            "id" => "BIGINT".to_string(),
            name if name.contains("count") || name.contains("amount") => "DECIMAL(10,2)".to_string(),
            name if name.ends_with("_at") => "TIMESTAMP".to_string(),
            name if name.ends_with("_date") => "DATE".to_string(),
            name if name.contains("email") => "VARCHAR(255)".to_string(),
            name if name.contains("url") || name.contains("link") => "VARCHAR(512)".to_string(),
            name if name.contains("description") || name.contains("content") => "TEXT".to_string(),
            _ => "VARCHAR(255)".to_string(),
        }
    }
}

