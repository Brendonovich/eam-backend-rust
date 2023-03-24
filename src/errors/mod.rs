use axum::{http::StatusCode, response::IntoResponse, Json};
use prisma_client_rust::QueryError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("failde to generate jwt")]
    TokenError(#[from] jsonwebtoken::errors::Error),
    // #[error("parse param error: {0}")]
    // AuthError(#[from] AuthError),
    #[error("database query error: {0}")]
    DbError(#[from] QueryError),
    #[error("failed due to: `{error:?}`")]
    Custom { status_code: u16, error: String },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (code, err) = match self {
            AppError::Custom { status_code, error } => {
                tracing::error!("return status code {} with error {}", status_code, error);
                (StatusCode::from_u16(status_code).unwrap(), error)
            }
            AppError::DbError(q) => {
                tracing::error!("an error occured during query execution: {}", q);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "query execution error".to_owned(),
                )
            }
            e => {
                tracing::error!("return status code 400 with error {}", e.to_string());
                (StatusCode::BAD_REQUEST, e.to_string())
            }
        };
        (
            code,
            Json(CommonResponse::<()> {
                error: Some(err),
                data: None,
            }),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(bound(deserialize = "T: Serialize + Deserialize<'de>"))]
    pub data: Option<T>,
}

impl<T> CommonResponse<T> {
    pub fn json_data(d: T) -> AppResult<Json<CommonResponse<T>>> {
        Ok(Json(CommonResponse {
            error: None,
            data: Some(d),
        }))
    }
}
