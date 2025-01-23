use std::fmt::Formatter;
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug, From)]
pub enum Error {
    Config(String),
    #[from]
    Request(reqwest::Error),
    #[from]
    JsonParse(serde_json::Error),
    Api(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}