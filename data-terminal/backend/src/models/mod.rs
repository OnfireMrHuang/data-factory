
pub mod error;
pub mod project;
pub mod resource;
pub mod datasource;
pub mod web;

pub use error::Error;
pub use web::{Response, PageQuery};
pub trait Validator {
    fn validate(&self) -> Result<(), Error>;
}