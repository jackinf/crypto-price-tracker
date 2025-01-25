//! Crate prelude

pub struct W<T>(pub T);
pub use crate::error::{Error, Result};

pub use log::debug as d;
pub use std::format as f;
