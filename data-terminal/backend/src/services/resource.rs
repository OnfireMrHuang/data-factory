use crate::repositories::{ResourceRepo};
use crate::models::{Error};
use crate::models::web::PageQuery;
use crate::models::resource::Resource;
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
    async fn add_resource(&self, mut resource: Resource) -> Result<String, Error> {
        // 设置默认值
        resource.created_at = chrono::Utc::now();
        resource.updated_at = chrono::Utc::now();
        
        let result = self.repo.add_resource(resource).await;
        match result {
            Ok(id) => Ok(id),
            Err(e) => Err(e),
        }
    }

    async fn edit_resource(&self, mut resource: Resource) -> Result<(), Error> {
        // 设置更新时间
        resource.updated_at = chrono::Utc::now();
        
        let result = self.repo.edit_resource(resource).await;
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

    async fn get_resource(&self, id: String) -> Result<Resource, Error> {
        let result = self.repo.get_resource(id).await;
        match result {
            Ok(resource) => Ok(resource),
            Err(e) => Err(e),
        }
    }

    async fn list_resource(&self, params: PageQuery) -> Result<Vec<Resource>, Error> {
        let result = self.repo.list_resource(params).await;
        match result {
            Ok(resources) => Ok(resources),
            Err(e) => Err(e),
        }
    }
}


