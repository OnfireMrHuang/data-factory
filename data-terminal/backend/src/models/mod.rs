
pub mod web;
pub mod project;
pub mod error;


pub use error::Error;
pub trait Validator {
    fn validate(&self) -> Result<(), Error>;
}