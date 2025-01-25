use base64::DecodeSliceError;
use derive_more::From;
use hmac::digest::InvalidLength;
use reqwest::header::InvalidHeaderValue;
use std::fmt::Formatter;

pub type Result<T> = core::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug, From)]
pub enum Error {
    Config(String),
    EnvNotSet(String),
    #[from]
    Request(reqwest::Error),
    #[from]
    JsonParse(serde_json::Error),
    ApiPayloadParseError,
    Api(String),
    NoSymbols,
    #[from]
    SignDecodeError(DecodeSliceError),
    #[from]
    SignInvalidHeaderError(InvalidHeaderValue),
    NewError,
    #[from]
    SerdeUrlEncodedError(serde_urlencoded::ser::Error),
    #[from]
    VarError(std::env::VarError),
    #[from]
    ParseFloatError(std::num::ParseFloatError),
    #[from]
    Hmac2InvalidLength(InvalidLength),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
