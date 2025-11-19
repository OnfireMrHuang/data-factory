//! API client modules for interacting with the backend
//!
//! This module contains all API calling code organized by domain:
//! - auth: Authentication operations
//! - projects: Project CRUD operations
//! - datasources: Datasource CRUD operations
//! - collections: Collection task operations
//! - resources: Resource CRUD operations
//! - test_run_mock: Mock API for test run functionality

pub mod client;
pub mod projects;
pub mod datasources;
pub mod collections;
pub mod resources;
pub mod test_run_mock;
