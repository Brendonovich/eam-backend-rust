use crate::errors::{AppError, AppResult, CommonResponse};
use crate::utils::JwtClaims;
use crate::DB;
use crate::{cmp_company_id, db};

use axum::debug_handler;
use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetCreateInfo {
    pub asset_code: String,
    pub asset_name: String,
    pub asset_description: String,
    pub asset_location_id: i32,
    pub asset_status_id: i32,
    pub customize_fileds_1: Option<String>,
    pub customize_fileds_2: Option<String>,
    pub customize_fileds_3: Option<String>,
    pub customize_fileds_4: Option<String>,
    pub customize_fileds_5: Option<String>,
}

#[debug_handler]
pub async fn create_asset(
    c: JwtClaims,
    Json(payload): Json<AssetCreateInfo>,
) -> AppResult<Json<CommonResponse<db::asset::Data>>> {
    let client = DB.get().unwrap();
    let a = client
        ._transaction()
        .run(|client| async move {
            client
                .asset()
                .create(
                    db::company::id::equals(c.company_id),
                    payload.asset_code,
                    payload.asset_name,
                    payload.asset_description,
                    db::asset_location::id::equals(payload.asset_location_id),
                    db::asset_status::id::equals(payload.asset_status_id),
                    vec![
                        db::asset::customize_fileds_1::set(payload.customize_fileds_1),
                        db::asset::customize_fileds_2::set(payload.customize_fileds_2),
                        db::asset::customize_fileds_3::set(payload.customize_fileds_3),
                        db::asset::customize_fileds_4::set(payload.customize_fileds_4),
                        db::asset::customize_fileds_5::set(payload.customize_fileds_5),
                    ],
                )
                .exec()
                .await
        })
        .await?;
    CommonResponse::json_data(a)
}

db::asset::select! {
    asset_out {
        id
        company_id
        asset_code
        asset_name
        asset_description
        asset_location: select {
            id
            location_code
            location_name
            location_description
        }
        asset_status: select {
            id
            status_code
            status_name
        }
        parent_asset
        children_asset
    }
}

#[debug_handler]
pub async fn get_asset(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<asset_out::Data>>> {
    let client = DB.get().unwrap();
    let a = client
        .asset()
        .find_unique(db::asset::id::equals(id))
        .select(asset_out::select())
        .exec()
        .await?;
    match a {
        Some(asset) => {
            cmp_company_id!(asset, c)
        }
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "asset not found".to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationCreateInfo {
    location_code: String,
    location_name: String,
    location_description: String,
    parent_id: Option<i32>,
}

#[debug_handler]
pub async fn create_location(
    c: JwtClaims,
    Json(payload): Json<LocationCreateInfo>,
) -> AppResult<Json<CommonResponse<db::asset_location::Data>>> {
    let client = DB.get().unwrap();
    let l = client
        .asset_location()
        .create(
            db::company::id::equals(c.company_id),
            payload.location_code,
            payload.location_name,
            payload.location_description,
            // vec![], 
            vec![db::asset_location::parent_id::set(payload.parent_id)],
        )
        .exec()
        .await?;
    CommonResponse::json_data(l)
}

db::asset_location::select! {
    location_out {
        id
        company_id
        location_code
        location_name
        location_description
    }
}

#[debug_handler]
pub async fn get_location(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<location_out::Data>>> {
    let client = DB.get().unwrap();
    let l = client
        .asset_location()
        .find_unique(db::asset_location::id::equals(id))
        .select(location_out::select())
        .exec()
        .await?;
    match l {
        Some(location) => cmp_company_id!(location, c),
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "location not found".to_string(),
        }),
    }
}

db::asset_location::select! {
    location_nested_out {
        id
        location_code
        location_name
        location_description
        parent_loaction
        children_location
    }
}

#[debug_handler]
pub async fn get_nested_location(
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<Vec<location_nested_out::Data>>>> {
    let client = DB.get().unwrap();
    CommonResponse::json_data(
        client
            .asset_location()
            .find_many(vec![db::asset_location::company_id::equals(c.company_id)])
            .select(location_nested_out::select())
            .exec()
            .await?,
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCreateInfo {
    pub status_code: String,
    pub status_name: String,
}

#[debug_handler]
pub async fn create_status(
    c: JwtClaims,
    Json(payload): Json<StatusCreateInfo>,
) -> AppResult<Json<CommonResponse<db::asset_status::Data>>> {
    let client = DB.get().unwrap();
    let s = client
        .asset_status()
        .create(
            db::company::id::equals(c.company_id),
            payload.status_code,
            payload.status_name,
            vec![],
        )
        .exec()
        .await?;
    CommonResponse::json_data(s)
}

#[debug_handler]
pub async fn get_status(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<db::asset_status::Data>>> {
    let client = DB.get().unwrap();
    let a = client
        .asset_status()
        .find_unique(db::asset_status::id::equals(id))
        .exec()
        .await?;
    match a {
        Some(s) => cmp_company_id!(s, c),
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "status not found".to_string(),
        }),
    }
}
