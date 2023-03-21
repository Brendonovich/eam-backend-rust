use anyhow::Result;
use axum::{debug_handler, extract::Path, Json};
use chrono::{Days, Local, Months};
use jsonwebtoken::{self};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::{debug, info};

use crate::{
    db::{self, company},
    errors::{AppError, AppResult, CommonResponse},
    utils::{JwtClaims, KEYS},
    DB,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisterInfo {
    pub company_code: String,
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub telephone: String,
    pub address: Option<String>,
    pub email: String,
    pub customize_fileds_1: Option<String>,
    pub customize_fileds_2: Option<String>,
    pub customize_fileds_3: Option<String>,
    pub customize_fileds_4: Option<String>,
    pub customize_fileds_5: Option<String>,
}

#[debug_handler]
pub async fn user_register(Json(ur): Json<UserRegisterInfo>) -> AppResult<()> {
    debug!("user register info: {:?}", ur);
    let client = DB.get().unwrap();
    let company = client
        .company()
        .find_unique(company::company_code::equals(ur.company_code.clone()))
        .exec()
        .await?;
    match company {
        Some(c) => {
            let admin_role = client
                .role()
                .find_first(vec![db::role::company_id::equals(c.id)])
                .exec()
                .await?;
            let role = match admin_role {
                Some(r) => r,
                _ => {
                    client
                        .role()
                        .create("admin".into(), company::id::equals(c.id), vec![])
                        .exec()
                        .await?
                }
            };
            let u = client
                .user()
                .create(
                    company::company_code::equals(ur.company_code),
                    crate::db::IsActive::Yes,
                    ur.username,
                    format!("{:X}", Sha256::digest(ur.password)),
                    ur.full_name,
                    ur.telephone,
                    ur.email,
                    db::role::id::equals(role.id),
                    vec![
                        db::user::customize_fileds_1::set(ur.customize_fileds_1),
                        db::user::customize_fileds_2::set(ur.customize_fileds_2),
                        db::user::customize_fileds_3::set(ur.customize_fileds_3),
                        db::user::customize_fileds_4::set(ur.customize_fileds_4),
                        db::user::customize_fileds_5::set(ur.customize_fileds_5),
                    ],
                )
                .exec()
                .await?;
            info!("successfully create user, id: {}", u.id);
            Ok(())
        }
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "company not found".to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginInfo {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginOutInfo {
    id: i32,
    token: String,
}

#[debug_handler]
pub async fn user_login(
    Json(ul): Json<UserLoginInfo>,
) -> AppResult<Json<CommonResponse<LoginOutInfo>>> {
    debug!("user login info: {:?}", ul);
    let encryped_password = format!("{:X}", Sha256::digest(ul.password));
    let client = DB.get().unwrap();
    let u = client
        .user()
        .find_first(vec![db::user::username::equals(ul.username)])
        .exec()
        .await?;
    match u {
        Some(user) => match user.is_active {
            db::IsActive::Yes => {
                debug!("find active user: {:?}", user);
                if encryped_password != user.password {
                    Err(AppError::Custom {
                        status_code: 403,
                        error: "wrong password".to_string(),
                    })
                } else {
                    let exp = Local::now() + Days::new(14);
                    let claim = JwtClaims {
                        user_id: user.id,
                        company_id: user.company_id,
                        role_id: user.role_id,
                        exp: exp.timestamp(),
                    };
                    let token = jsonwebtoken::encode(
                        &jsonwebtoken::Header::default(),
                        &claim,
                        &KEYS.encoding,
                    )?;
                    Ok(Json(CommonResponse {
                        data: Some(LoginOutInfo { token, id: user.id }),
                        error: None,
                    }))
                }
            }
            _ => Err(AppError::Custom {
                status_code: 403,
                error: "user is not active".to_string(),
            }),
        },
        _ => Err(AppError::Custom {
            status_code: 400,
            error: "user does not exist".to_string(),
        }),
    }
}

db::user::select! { user_out {
    id
    company: select {
        id
        company_name
        company_code
        expiration_date
        customize_fileds_1
        customize_fileds_2
        customize_fileds_3
        customize_fileds_4
        customize_fileds_5
    }
    username
    email
    telephone
    address
    role: select {
        id
        role_name
    }
    customize_fileds_1
    customize_fileds_2
    customize_fileds_3
    customize_fileds_4
    customize_fileds_5
}}

pub async fn user_details(
    Path(id): Path<i32>,
    c: JwtClaims,
) -> AppResult<Json<CommonResponse<user_out::Data>>> {
    let client = DB.get().unwrap();
    let u = client
        .user()
        .find_first(vec![
            db::user::id::equals(id),
            db::user::company_id::equals(c.company_id),
        ])
        .select(user_out::select())
        .exec()
        .await?;
    match u {
        Some(user) => CommonResponse::json_data(user),
        _ => Err(AppError::Custom {
            status_code: 404,
            error: "user not found".to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserInfo {
    pub full_name: Option<String>,
    pub address: Option<String>,
    pub password: Option<String>,
    pub telephone: Option<String>,
    pub customize_fileds_1: Option<String>,
    pub customize_fileds_2: Option<String>,
    pub customize_fileds_3: Option<String>,
    pub customize_fileds_4: Option<String>,
    pub customize_fileds_5: Option<String>,
    pub role_id: Option<i32>,
}

pub async fn update_user(
    Path(id): Path<i32>,
    c: JwtClaims,
    Json(payload): Json<UpdateUserInfo>,
) -> AppResult<Json<CommonResponse<user_out::Data>>> {
    let client = DB.get().unwrap();
    if id == c.user_id {
        CommonResponse::json_data(
            client
                .user()
                .update(db::user::id::equals(id), vec![
                    // db::user::full_name::set(payload.full_name)
                ])
                .select(user_out::select())
                .exec()
                .await?,
        )
    } else {
        let r = client
            .role()
            .find_unique(db::role::id::equals(c.role_id))
            .exec()
            .await?;
        if let Some(role) = r {
            if let Some(rpv) = role.role_privileges {
                for p in rpv {
                    if p.module == db::Module::Admin && p.privilege_type == db::PrivilegeType::Edit
                    {
                    }
                }
            }
        }
        todo!()
    }
}

#[derive(Debug, Clone, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct CompanyRegisterInfo {
    pub company_code: String,
    pub company_name: String,
    pub company_bid: String,
    pub email: String,
    pub telephone: String,
    pub address: Option<String>,
    pub customize_fileds_1: Option<String>,
    pub customize_fileds_2: Option<String>,
    pub customize_fileds_3: Option<String>,
    pub customize_fileds_4: Option<String>,
    pub customize_fileds_5: Option<String>,
}

pub async fn company_register(Json(cr): Json<CompanyRegisterInfo>) -> AppResult<()> {
    debug!("{:?}", cr);
    let client = DB.get().unwrap();
    let find = client
        .company()
        .find_unique(db::company::company_code::equals(cr.company_code.clone()))
        .exec()
        .await?;
    if let Some(_) = find {
        Err(AppError::Custom {
            status_code: 400,
            error: "duplicate company code".to_string(),
        })
    } else {
        let expire_date = Local::now() + Months::new(12);
        debug!("{:?}", expire_date);
        client
            .company()
            .create(
                cr.company_code,
                cr.company_name,
                cr.email,
                cr.telephone,
                cr.company_bid,
                expire_date.into(),
                vec![],
            )
            .exec()
            .await?;
        Ok(())
    }
}
