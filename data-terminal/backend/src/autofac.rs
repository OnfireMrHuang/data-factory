use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use shaku::{module, HasProvider};
use crate::repositories::project::ProjectRepoImpl;
use crate::repositories::resource::ResourceRepoImpl;
use crate::repositories::datasource::DataSourceRepoImpl;
use crate::repositories::collection_task::CollectionRepositoryImpl;
use crate::services::project::ProjectServiceImpl;
use crate::services::resource::ResourceServiceImpl;
use crate::services::datasource::DataSourceServiceImpl;
use crate::services::collection_service::CollectionServiceImpl;
use crate::services::{ProjectService, ResourceService, DataSourceService};
use crate::services::collection_service::CollectionService;

module! {
    pub AutoFacModule {
        components = [],
        providers = [ProjectRepoImpl, ProjectServiceImpl,
        ResourceRepoImpl, ResourceServiceImpl,
        DataSourceRepoImpl, DataSourceServiceImpl,
        CollectionRepositoryImpl, CollectionServiceImpl]
    }
}

// 全局单例容器
static GLOBAL_APP_STATE: Lazy<Arc<Mutex<AppState>>> = Lazy::new(|| {
    Arc::new(Mutex::new(create_app_state()))
});

/// 创建 AppState 实例
fn create_app_state() -> AppState {
    let module = Arc::new(
        AutoFacModule::builder()
            .build(),
    );
    AppState { module }
}

/// 获取全局 AppState 实例
pub fn get_global_app_state() -> Arc<Mutex<AppState>> {
    GLOBAL_APP_STATE.clone()
}

/// 获取全局 AppState 的只读引用（用于不需要修改的场景）
pub fn get_global_app_state_ref() -> AppState {
    let state = GLOBAL_APP_STATE.lock().unwrap();
    state.clone()
}

/// 初始化全局 AppState（在 main 函数中调用）
pub fn init_global_app_state() {
    // Lazy 会在第一次访问时自动初始化
    // 这里只是确保初始化发生
    let _ = get_global_app_state();
}



#[derive(Clone)]
pub struct AppState {
    module: Arc<AutoFacModule>,
}

impl AppState {
    pub fn get_auto_fac_module(&self) -> Arc<AutoFacModule> {
        self.module.clone()
    }

    /// 获取 ProjectService 实例
    pub fn get_project_service(&self) -> Box<dyn ProjectService> {
        self.module.provide().unwrap()
    }

    /// 获取 ResourceService 实例
    pub fn get_resource_service(&self) -> Box<dyn ResourceService> {
        self.module.provide().unwrap()
    }

    /// 获取 DataSourceService 实例
    pub fn get_datasource_service(&self) -> Box<dyn DataSourceService> {
        self.module.provide().unwrap()
    }

    /// 获取 CollectionService 实例
    pub fn get_collection_service(&self) -> Box<dyn CollectionService> {
        self.module.provide().unwrap()
    }
}
