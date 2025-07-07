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

#[derive(Clone)]
pub struct AppState {
    module: Arc<AutoFacModule>,
}

impl AppState {   
    pub fn get_auto_fac_module(&self) -> Arc<AutoFacModule> {
        self.module.clone()
    }

    /// 获取 ProjectRepo 实例
    pub fn get_project_repo(&self) -> Box<dyn ProjectRepo> {
        self.module.provide().unwrap()
    }

    /// 获取 ProjectService 实例
    pub fn get_project_service(&self) -> Box<dyn ProjectService> {
        self.module.provide().unwrap()
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

/// 便捷方法：获取 ProjectRepo
pub fn get_project_repo_global() -> Box<dyn ProjectRepo> {
    let state = GLOBAL_APP_STATE.lock().unwrap();
    state.get_project_repo()
}

/// 便捷方法：获取 ProjectService
pub fn get_project_service_global() -> Box<dyn ProjectService> {
    let state = GLOBAL_APP_STATE.lock().unwrap();
    state.get_project_service()
}

// 为了兼容 axum 的 State 提取器
impl FromRef<AppState> for Arc<AutoFacModule> {
    fn from_ref(state: &AppState) -> Self {
        state.get_auto_fac_module()
    }
}

/*
使用示例：

1. 在 routes 中使用：
```rust
use crate::autofac;

async fn some_handler() -> impl Responder {
    // 获取 ProjectService
    let project_service = autofac::get_project_service_global();
    let result = project_service.add_project(project).await;
    // ...
}
```

2. 在 services 中使用：
```rust
use crate::autofac;

pub struct SomeServiceImpl;

impl SomeService for SomeServiceImpl {
    async fn some_method(&self) -> Result<(), Error> {
        // 获取 ProjectRepo
        let project_repo = autofac::get_project_repo_global();
        let result = project_repo.list_project().await;
        // ...
    }
}
```

3. 在 repositories 中使用：
```rust
use crate::autofac;

pub struct SomeRepoImpl;

impl SomeRepo for SomeRepoImpl {
    async fn some_method(&self) -> Result<(), Error> {
        // 获取其他依赖
        let other_service = autofac::get_project_service_global();
        // ...
    }
}
```

4. 获取完整的 AppState：
```rust
use crate::autofac;

fn some_function() {
    // 获取完整的 AppState（需要锁）
    let app_state = autofac::get_global_app_state();
    let state = app_state.lock().unwrap();
    let service = state.get_project_service();
    
    // 或者获取只读引用（不需要锁）
    let state_ref = autofac::get_global_app_state_ref();
    let service = state_ref.get_project_service();
}
```

注意事项：
- 全局 AppState 使用 Mutex 保证并发安全
- 使用 Lazy 确保单例模式，只在第一次访问时初始化
- 在 main 函数中调用 init_global_app_state() 确保初始化
- 所有模块都可以通过 autofac 模块访问全局状态
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_app_state_singleton() {
        // 测试单例模式
        let state1 = get_global_app_state();
        let state2 = get_global_app_state();
        
        // 应该返回相同的 Arc 引用
        assert!(Arc::ptr_eq(&state1, &state2));
    }

    #[test]
    fn test_get_project_service() {
        // 测试获取 ProjectService
        let _service = get_project_service_global();
        // 验证可以正常获取服务实例
    }

    #[test]
    fn test_get_project_repo() {
        // 测试获取 ProjectRepo
        let _repo = get_project_repo_global();
        // 验证可以正常获取仓库实例
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;
        use std::sync::Arc;
        
        // 测试并发访问
        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(|| {
                    let _service = get_project_service_global();
                    let _repo = get_project_repo_global();
                    // 验证可以正常获取实例
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}



