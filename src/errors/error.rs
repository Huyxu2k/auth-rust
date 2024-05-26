use std::time::SystemTime;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;


#[derive(Debug,Serialize)]
pub enum TypeError {
    NotFound,
    Forbidden,
    InternalServerError,
    ServiceUnavailable,
    BadRequest,
    Unauthorized,
    MethodNotAllowed,
    Redirect,
}

impl TypeError {
    fn status_code(&self) -> u16 {
        match self {
            TypeError::NotFound => 404,
            TypeError::Forbidden => 403,
            TypeError::InternalServerError => 500,
            TypeError::ServiceUnavailable => 503,
            TypeError::BadRequest => 400,
            TypeError::Unauthorized => 401,
            TypeError::MethodNotAllowed => 405,
            TypeError::Redirect => 302,
        }
    }
}

#[derive(Serialize,ToSchema)]
pub struct HttpError{
    message: String,
    timestamp: String,
    #[serde(rename(serialize="errorCode", deserialize="errorCode"))]
    error_code: u16,
}

impl HttpError {
    pub fn new(message: &str, error: TypeError)->HttpError{
        let now: DateTime<Utc>=SystemTime::now().into();
        let now: String= now.to_rfc3339();

        HttpError { 
            message: String::from(message), 
            timestamp: now, 
            error_code: error.status_code()
        }
    }
}

