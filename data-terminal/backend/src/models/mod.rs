
pub mod web;
pub mod project;
pub mod error;
pub mod resource;


pub use error::Error;
pub trait Validator {
    fn validate(&self) -> Result<(), Error>;
}