pub mod project;
pub mod resource;
pub mod datasource;
pub mod collection_service;

use async_trait::async_trait;
use crate::models::project::Project;
use crate::models::error::Error;
use crate::models::web::PageQuery;
use crate::models::resource::{Resource, ResourceReadOnly, ResourceCreateUpdate};
use crate::models::datasource::{DataSource, DataSourceReadOnly, DataSourceCreateUpdate};
use crate::models::collection::{
    TableMetadata, FieldMetadata, CollectTaskReadOnly, CreateCollectTaskRequest,
    UpdateCollectTaskRequest, TaskStage, CollectionCategory, CollectType, TableSchema, TableSelection, CollectTask
};

#[async_trait]
pub trait ProjectService: Send {
    async fn add_project(&self, project: Project) -> Result<String, Error>;
    async fn edit_project(&self, project: Project) -> Result<(), Error>;
    async fn del_project(&self, code: String) -> Result<(), Error>;
    async fn get_project(&self, code: String) -> Result<Project, Error>;
    async fn list_project(&self, params: PageQuery) -> Result<Vec<Project>, Error>;
}

#[async_trait]
pub trait ResourceService: Send {
    async fn add_resource(&self, resource: ResourceCreateUpdate) -> Result<String, Error>;
    async fn edit_resource(&self, resource: ResourceCreateUpdate) -> Result<(), Error>;
    async fn del_resource(&self, id: String) -> Result<(), Error>;
    async fn get_resource(&self, id: String) -> Result<ResourceReadOnly, Error>;
    async fn list_resource(&self, params: PageQuery) -> Result<Vec<ResourceReadOnly>, Error>;
}

#[async_trait]
pub trait DataSourceService: Send {
    async fn add_datasource(&self, project_code: String, datasource: DataSourceCreateUpdate) -> Result<String, Error>;
    async fn edit_datasource(&self, project_code: String, datasource: DataSourceCreateUpdate) -> Result<(), Error>;
    async fn ping_datasource(&self, project_code: String, datasource: DataSourceCreateUpdate) -> Result<(), Error>;
    async fn del_datasource(&self, project_code: String, id: String) -> Result<(), Error>;
    async fn get_datasource(&self, project_code: String, id: String) -> Result<DataSourceReadOnly, Error>;
    async fn list_datasource(&self, project_code: String, params: PageQuery) -> Result<Vec<DataSourceReadOnly>, Error>;
    async fn get_datasource_tables(&self, project_code: String, datasource_id: String) -> Result<Vec<TableMetadata>, Error>;
    async fn get_table_fields(&self, project_code: String, datasource_id: String, table_name: String) -> Result<Vec<FieldMetadata>, Error>;
}

#[async_trait]
pub trait CollectionService: Send {
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
        datasource_id: &str,
        resource_id: &str,
        selected_tables: Vec<TableSelection>,
    ) -> Result<TableSchema, Error>;

    async fn validate_task_config(&self, task: &CollectTask) -> Result<(), Error>;
}
