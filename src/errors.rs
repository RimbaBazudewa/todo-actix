use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use deadpool_postgres::PoolError;
use serde::Serialize;
use std::fmt;
use tokio_postgres::Error;
#[derive(Debug)]
pub enum AppErrorType {
    DBError,
    NotFoundError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                message: None,
                error_type: AppErrorType::NotFoundError,
                ..
            } => "The request item was not found ".to_string(),
            _ => "An unexpected erorr has occured".to_string(),
        }
    }
}
impl From<PoolError> for AppError {
    fn from(error: PoolError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DBError,
        }
    }
}
impl From<Error> for AppError {
    fn from(error: Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DBError,
        }
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}
#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.error_type {
            AppErrorType::DBError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{AppError, AppErrorType};

    #[test]
    fn test_default_message() {
        let db_error: AppError = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::DBError,
        };

        assert_eq!(
            db_error.message(),
            "An unexpected erorr has occured".to_string(),
            "Default message should be shown"
        )
    }
    #[test]
    fn test_custom_message() {
        let custom_message = "unable to create item ".to_string();
        let db_error = AppError {
            message: Some(custom_message.clone()),
            cause: None,
            error_type: AppErrorType::DBError,
        };
        assert_eq!(
            db_error.message(),
            custom_message,
            "User-facing  message should be shown"
        )
    }
}
