
pub mod error;
pub mod project;
pub mod datasource;
pub mod resource;
pub mod web;

pub use error::Error;

pub trait Validator {
    fn validate(&self) -> Result<(), Error>;
}