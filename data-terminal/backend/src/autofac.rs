use std::sync::{Arc, Mutex};
use std::any::Any;
use once_cell::sync::Lazy;
use axum::extract::FromRef;
use shaku::{module, HasComponent, HasProvider};
use crate::repositories::project::ProjectRepoImpl;
use crate::services::project::ProjectServiceImpl;
use crate::repositories::ProjectRepo;
use crate::services::ProjectService;

module! {
    pub AutoFacModule {
        components = [],
        providers = [ProjectRepoImpl, ProjectServiceImpl]
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
}
