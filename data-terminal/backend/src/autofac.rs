
use std::sync::Arc;
use axum::extract::FromRef;
use shaku::{module};
use crate::repositories::project::ProjectRepoImpl;
use crate::services::project::ProjectServiceImpl;

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
}


pub fn get_app_state() -> AppState {
    let module = Arc::new(
        AutoFacModule::builder()
            .build(),
    );
    AppState { module }
}



