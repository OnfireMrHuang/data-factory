use std::collections::HashMap;

use async_trait::async_trait;
use chrono::Utc;
use shaku::Interface;
use uuid::Uuid;
use shaku::Provider;

use crate::models::collection::*;
use crate::models::datasource::DataSourceReadOnly;
use crate::models::resource::ResourceReadOnly;
use crate::repositories::collection_task::CollectionRepository;
use crate::models::Error;
use crate::services::{DataSourceService, ResourceService};
use super::CollectionService;

/// Collection service implementation
#[derive(Provider)]
#[shaku(interface = CollectionService)]
pub struct CollectionServiceImpl {
    #[shaku(provide)]
    repository: Box<dyn CollectionRepository>,
    #[shaku(provide)]
    datasource_service: Box<dyn DataSourceService>,
    #[shaku(provide)]
    resource_service: Box<dyn ResourceService>,
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
        request: CreateOrUpdateCollectTaskRequest,
    ) -> Result<CollectTaskReadOnly, Error> {
        // Generate UUID for new task
        let id = Uuid::new_v4().to_string();
        let code = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Validate compatibility with comprehensive rule checking
        if request.rule.is_some() {
            self.validate_compatibility(&request.category, &request.collect_type, &request.rule.as_ref().unwrap())?;
        }

        let task = CollectTask {
            id,
            code,
            name: request.name,
            description: request.description,
            category: request.category,
            collect_type: request.collect_type,
            datasource_id: request.datasource_id,
            queue_resource_id: request.queue_resource_id,
            database_resource_id: request.database_resource_id,
            rule: request.rule.unwrap_or_default(),
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

    async fn get_task(&self, project_code: String, params: DetailRequest) -> Result<Option<CollectTaskReadOnly>, Error> {
        let task = self.repository.find_by_code(project_code, &params.code, params.stage).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to get task: {:?}", e)),
            })?;

        Ok(task.map(CollectTaskReadOnly::from))
    }

    async fn update_task(
        &self,
        project_code: String,
        request: CreateOrUpdateCollectTaskRequest,
    ) -> Result<CollectTaskReadOnly, Error> {
        // Fetch existing task
        let mut task = self.repository.find_by_code(project_code.clone(), &request.code, TaskStage::Draft).await
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

        task.name = request.name;
        task.description = request.description;
        task.category = request.category;
        task.collect_type = request.collect_type;
        task.datasource_id = request.datasource_id;
        task.queue_resource_id = request.queue_resource_id;
        task.database_resource_id = request.database_resource_id;
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

    async fn delete_task(&self, project_code: String, code: &str) -> Result<(), Error> {
        // Fetch task to check status
        let task = self.repository.find_by_code(project_code.clone(), code, TaskStage::Draft).await
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

        let total = self.repository.count_all(project_code.clone(), stage, category, collect_type).await
            .map_err(|e| match e {
                Error::DbError(_) => e,
                _ => Error::InternalError(format!("Failed to count tasks: {:?}", e)),
            })?;

        // 批量获取数据源信息
        let datasource_list = self
            .datasource_service
            .batch_query_datsource(
                project_code.clone(),
                tasks
                    .iter()
                    .map(|task| task.datasource_id.clone())
                    .collect(),
            )
            .await
            .map_err(|e| Error::InternalError(format!("Failed to query datasources: {:?}", e)))?;

        let datasource_map: HashMap<String, DataSourceReadOnly> = datasource_list
            .into_iter()
            .map(|ds| (ds.id.clone(), ds))
            .collect();

        // 批量获取资源信息 (收集所有的 queue_resource_id 和 database_resource_id)
        let mut all_resource_ids: Vec<String> = tasks
            .iter()
            .flat_map(|task| {
                let mut ids = Vec::new();
                if !task.queue_resource_id.is_empty() {
                    ids.push(task.queue_resource_id.clone());
                }
                if !task.database_resource_id.is_empty() {
                    ids.push(task.database_resource_id.clone());
                }
                ids
            })
            .collect();

        // 去重
        all_resource_ids.sort();
        all_resource_ids.dedup();

        let resource_list = if !all_resource_ids.is_empty() {
            self
                .resource_service
                .batch_query_resource(all_resource_ids)
                .await
                .map_err(|e| Error::InternalError(format!("Failed to query resources: {:?}", e)))?
        } else {
            Vec::new()
        };

        let resource_map: HashMap<String, ResourceReadOnly> = resource_list
            .into_iter()
            .map(|res| (res.id.clone(), res))
            .collect();

        Ok((tasks.into_iter().map(|task| {
            let mut item = CollectTaskReadOnly::from(task);
            let datasource = datasource_map.get(&item.datasource_id).cloned();

            // 设置队列资源名称
            if !item.queue_resource_id.is_empty() {
                if let Some(resource) = resource_map.get(&item.queue_resource_id) {
                    item.queue_resource_name = resource.name.clone();
                }
            }

            // 设置数据库资源名称
            if !item.database_resource_id.is_empty() {
                if let Some(resource) = resource_map.get(&item.database_resource_id) {
                    item.database_resource_name = resource.name.clone();
                }
            }

            if let Some(datasource) = datasource {
                item.datasource_name = datasource.name;
            }
            item
        }).collect(), total))
    }

    async fn apply_task(&self, project_code: String, code: &str) -> Result<CollectTaskReadOnly, Error> {
        // Fetch task
        let mut task = self.repository.find_by_code(project_code.clone(), code, TaskStage::Draft).await
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

