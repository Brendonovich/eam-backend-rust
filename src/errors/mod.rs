use async_trait::async_trait;
use prisma_client_rust::QueryError;
use salvo::jwt_auth::JwtError;
use salvo::{
    http::ParseError, prelude::StatusCode, writer::Json, Depot, Request, Response, Writer,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("failde to generate jwt")]
    TokenError(#[from] JwtError),
    #[error("parse param error: {0}")]
    ParamParseError(#[from] ParseError),
    #[error("database query error: {0}")]
    DbError(#[from] QueryError),
    #[error("failed due to: `{error:?}`")]
    Custom { status_code: u16, error: String },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            AppError::Custom { status_code, error } => {
                res.set_status_code(StatusCode::from_u16(status_code).unwrap());
                res.render(Json(CommonResponse::<()> {
                    error: Some(error),
                    data: None,
                }));
            }
            AppError::DbError(q) => {
                res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
                tracing::error!("an error occured during query execution: {}", q);
                res.render(Json(CommonResponse::<()> {
                    error: Some("query execution error".to_string()),
                    data: None,
                }))
            }
            AppError::Other(e) => {
                res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(CommonResponse::<()> {
                    error: Some(e.to_string()),
                    data: None,
                }));
            }
            e => {
                res.set_status_code(StatusCode::BAD_REQUEST);
                res.render(Json(CommonResponse::<()> {
                    error: Some(e.to_string()),
                    data: None,
                }));
            }
        }
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
