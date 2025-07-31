use crate::repositories::{ResourceRepo};
use crate::models::{Error};
use crate::models::web::PageQuery;
use crate::models::resource::{Resource, ResourceReadOnly, ResourceCreateUpdate};
use shaku::Provider;
use async_trait::async_trait;
use super::ResourceService;
use chrono;

#[derive(Provider)]
#[shaku(interface = ResourceService)]
pub struct ResourceServiceImpl {
    #[shaku(provide)]
    repo: Box<dyn ResourceRepo>,
}

#[async_trait]
impl ResourceService for ResourceServiceImpl {
    async fn add_resource(&self, resource: ResourceCreateUpdate) -> Result<String, Error> {
        let resource = Resource::from(resource);
        let result = self.repo.add_resource(resource).await;
        match result {
            Ok(id) => Ok(id),
            Err(e) => Err(e),
        }
    }

    async fn edit_resource(&self, resource: ResourceCreateUpdate) -> Result<(), Error> {
        // 获取现有资源以保留状态
        let existing = self.repo.get_resource(resource.id.clone()).await?;
        let mut updated_resource = Resource::from(resource);
        updated_resource.status = existing.status;
        updated_resource.created_at = existing.created_at;
        updated_resource.updated_at = chrono::Utc::now();
        
        let result = self.repo.edit_resource(updated_resource).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn del_resource(&self, id: String) -> Result<(), Error> {
        let result = self.repo.del_resource(id).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn get_resource(&self, id: String) -> Result<ResourceReadOnly, Error> {
        let result = self.repo.get_resource(id).await;
        match result {
            Ok(resource) => Ok(ResourceReadOnly::from(resource)),
            Err(e) => Err(e),
        }
    }

    async fn list_resource(&self, params: PageQuery) -> Result<Vec<ResourceReadOnly>, Error> {
        let result = self.repo.list_resource(params).await;
        match result {
            Ok(resources) => Ok(resources.into_iter().map(ResourceReadOnly::from).collect()),
            Err(e) => Err(e),
        }
    }
}


