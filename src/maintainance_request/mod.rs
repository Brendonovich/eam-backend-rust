use crate::errors::{AppError, AppResult, CommonResponse};
use crate::utils::JwtClaims;
use crate::DB;
use crate::{cmp_company_id, db};

use axum::debug_handler;
use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMrInfo {
    asset_id: i32,
    mr_name: String,
    mr_status_id: i32,
    mr_category_id: i32,
    mr_priority_id: i32,
    mr_failure_impact_id: i32,
    mr_failure_mode_id: i32,
    mr_error_code: String,
    mr_description: String,
}

#[debug_handler]
pub async fn create_mr(
    c: JwtClaims,
    Json(payload): Json<CreateMrInfo>,
) -> AppResult<Json<CommonResponse<db::maintainance_request::Data>>> {
    let client = DB.get().unwrap();
    CommonResponse::json_data(
        client
            .maintainance_request()
            .create(
                db::company::id::equals(c.company_id),
                db::asset::id::equals(payload.asset_id),
                payload.mr_name,
                db::user::id::equals(c.user_id),
                db::mr_status::id::equals(payload.mr_status_id),
                db::mr_category::id::equals(payload.mr_category_id),
                db::mr_priority::id::equals(payload.mr_priority_id),
                db::mr_failure_impact::id::equals(payload.mr_failure_impact_id),
                db::mr_failure_mode::id::equals(payload.mr_failure_mode_id),
                payload.mr_error_code,
                payload.mr_description,
                vec![],
            )
            .exec()
            .await?,
    )
}

#[debug_handler]
pub async fn get_mr(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<db::maintainance_request::Data>>> {
    let client = DB.get().unwrap();
    let a = client
        .maintainance_request()
        .find_unique(db::maintainance_request::id::equals(id))
        .exec()
        .await?;
    match a {
        Some(s) => cmp_company_id!(s, c),
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "mr not found".to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMrStatusInfo {
    pub status_code: String,
    pub status_name: String,
}

#[debug_handler]
pub async fn create_mr_status(
    c: JwtClaims,
    Json(payload): Json<CreateMrStatusInfo>,
) -> AppResult<Json<CommonResponse<db::mr_status::Data>>> {
    let client = DB.get().unwrap();
    CommonResponse::json_data(
        client
            .mr_status()
            .create(
                db::company::id::equals(c.company_id),
                payload.status_code,
                payload.status_name,
                vec![],
            )
            .exec()
            .await?,
    )
}

#[debug_handler]
pub async fn get_mr_status(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<db::mr_status::Data>>> {
    let client = DB.get().unwrap();
    let a = client
        .mr_status()
        .find_unique(db::mr_status::id::equals(id))
        .exec()
        .await?;
    match a {
        Some(s) => cmp_company_id!(s, c),
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "mr not found".to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMrCategoryInfo {
    pub category_code: String,
    pub category_name: String,
}

#[debug_handler]
pub async fn create_mr_category(
    c: JwtClaims,
    Json(payload): Json<CreateMrCategoryInfo>,
) -> AppResult<Json<CommonResponse<db::mr_category::Data>>> {
    let client = DB.get().unwrap();
    CommonResponse::json_data(
        client
            .mr_category()
            .create(
                db::company::id::equals(c.company_id),
                payload.category_code,
                payload.category_name,
                vec![],
            )
            .exec()
            .await?,
    )
}

#[debug_handler]
pub async fn get_mr_category(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<db::mr_category::Data>>> {
    let client = DB.get().unwrap();
    let a = client
        .mr_category()
        .find_unique(db::mr_category::id::equals(id))
        .exec()
        .await?;
    match a {
        Some(s) => cmp_company_id!(s, c),
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "mr not found".to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMrPriorityInfo {
    pub priority_code: String,
    pub priority_name: String,
}

#[debug_handler]
pub async fn create_mr_priority(
    c: JwtClaims,
    Json(payload): Json<CreateMrPriorityInfo>,
) -> AppResult<Json<CommonResponse<db::mr_priority::Data>>> {
    let client = DB.get().unwrap();
    CommonResponse::json_data(
        client
            .mr_priority()
            .create(
                db::company::id::equals(c.company_id),
                payload.priority_code,
                payload.priority_name,
                vec![],
            )
            .exec()
            .await?,
    )
}

#[debug_handler]
pub async fn get_mr_priority(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<db::mr_priority::Data>>> {
    let client = DB.get().unwrap();
    let a = client
        .mr_priority()
        .find_unique(db::mr_priority::id::equals(id))
        .exec()
        .await?;
    match a {
        Some(s) => cmp_company_id!(s, c),
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "mr not found".to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMrFailureImpactInfo {
    pub failure_impact_code: String,
    pub failure_impact_name: String,
}

#[debug_handler]
pub async fn create_mr_failure_impact(
    c: JwtClaims,
    Json(payload): Json<CreateMrFailureImpactInfo>,
) -> AppResult<Json<CommonResponse<db::mr_failure_impact::Data>>> {
    let client = DB.get().unwrap();
    CommonResponse::json_data(
        client
            .mr_failure_impact()
            .create(
                db::company::id::equals(c.company_id),
                payload.failure_impact_code,
                payload.failure_impact_name,
                vec![],
            )
            .exec()
            .await?,
    )
}

#[debug_handler]
pub async fn get_mr_failure_impact(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<db::mr_failure_impact::Data>>> {
    let client = DB.get().unwrap();
    let a = client
        .mr_failure_impact()
        .find_unique(db::mr_failure_impact::id::equals(id))
        .exec()
        .await?;
    match a {
        Some(s) => cmp_company_id!(s, c),
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "mr not found".to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMrFailureModeInfo {
    pub failure_mode_code: String,
    pub failure_mode_name: String,
}

#[debug_handler]
pub async fn create_mr_failure_mode(
    c: JwtClaims,
    Json(payload): Json<CreateMrFailureModeInfo>,
) -> AppResult<Json<CommonResponse<db::mr_failure_mode::Data>>> {
    let client = DB.get().unwrap();
    CommonResponse::json_data(
        client
            .mr_failure_mode()
            .create(
                db::company::id::equals(c.company_id),
                payload.failure_mode_code,
                payload.failure_mode_name,
                vec![],
            )
            .exec()
            .await?,
    )
}

#[debug_handler]
pub async fn get_mr_failure_mode(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<db::mr_failure_mode::Data>>> {
    let client = DB.get().unwrap();
    let a = client
        .mr_failure_mode()
        .find_unique(db::mr_failure_mode::id::equals(id))
        .exec()
        .await?;
    match a {
        Some(s) => cmp_company_id!(s, c),
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "mr not found".to_string(),
        }),
    }
}
