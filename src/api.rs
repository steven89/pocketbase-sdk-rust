use std::error::Error;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum ApiErrorCode {
    BadRequest = 400,
    Forbidden = 403,
    NotFound = 404,
}

impl From<ApiErrorCode> for http::StatusCode {
    fn from(value: ApiErrorCode) -> Self {
        match value {
            ApiErrorCode::BadRequest => http::StatusCode::BAD_REQUEST,
            ApiErrorCode::Forbidden => http::StatusCode::FORBIDDEN,
            ApiErrorCode::NotFound => http::StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiError {
    pub code: ApiErrorCode,
    pub message: String,
    pub data: serde_json::Map<String, serde_json::Value>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code {
            ApiErrorCode::BadRequest => write!(f, "Bad Request"),
            ApiErrorCode::Forbidden => write!(f, "Forbidden"),
            ApiErrorCode::NotFound => write!(f, "Not Found"),
        }
    }
}

impl Error for ApiError {}

#[derive(Debug)]
pub enum RequestError {
    Url(url::ParseError),
    Reqwest(reqwest::Error),
    Api(ApiError),
    ParseError(serde_json::Error, String)
}

impl From<url::ParseError> for RequestError {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value)
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<ApiError> for RequestError {
    fn from(value: ApiError) -> Self {
        Self::Api(value)
    }
}
