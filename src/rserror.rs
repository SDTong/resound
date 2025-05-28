//! error

use std::result;

pub(crate) type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub(crate) type Result<T> = result::Result<T, GenericError>;
