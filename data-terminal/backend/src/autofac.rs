
use std::sync::Arc;
use axum::extract::FromRef;
use shaku::{module, Component, Interface};

module! {
    pub AutoFacModule {
        components = [],
        providers = []
    }
}

#[derive(Clone)]
pub struct AppState {
    module: Arc<AutoFacModule>,
}

impl FromRef<AppState> for Arc<AutoFacModule> {
    fn from_ref(app_state: &AppState) -> Arc<AutoFacModule> {
        app_state.module.clone()
    }
}


pub fn get_app_state() -> AppState {
    let module = Arc::new(
        AutoFacModule::builder()
            .build(),
    );
    AppState { module }
}



