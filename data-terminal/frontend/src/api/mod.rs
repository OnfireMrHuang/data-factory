//! API client modules for interacting with the backend
//!
//! This module contains all API calling code organized by domain:
//! - auth: Authentication operations
//! - projects: Project CRUD operations
//! - datasources: Datasource CRUD operations
//! - collections: Collection task operations
//! - resources: Resource CRUD operations

pub mod client;
pub mod projects;
pub mod datasources;
pub mod collections;
pub mod resources;
