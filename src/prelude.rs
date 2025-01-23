//! Crate prelude

#[allow(unused)]
pub struct W<T>(pub T);
pub use crate::error::{Error, Result};

#[allow(unused)]
pub use std::format as f;
pub use log::debug as d;