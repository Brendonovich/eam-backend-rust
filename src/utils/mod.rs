use std::fmt::Debug;

use anyhow::{bail, Result};
use async_trait::async_trait;
use axum::{
    extract::{rejection::TypedHeaderRejectionReason, FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{
    db::{self, PrivilegeType},
    errors::AppError,
    DB,
};

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub user_id: i32,
    pub company_id: i32,
    pub role_id: i32,
    pub exp: i64,
}

impl JwtClaims {
    pub async fn check_module_privilige(
        &self,
        module: db::Module,
        privilege: db::PrivilegeType,
    ) -> Result<()> {
        let client = DB.get().unwrap();
        let r = client
            .role()
            .find_unique(db::role::id::equals(self.role_id))
            .exec()
            .await?;
        if let Some(role) = r {
            if let Some(rpv) = role.role_privileges {
                for p in rpv {
                    if p.module == module && p.privilege_type == privilege {
                        match privilege {
                            PrivilegeType::Edit => return Ok(()),
                            PrivilegeType::View => match p.privilege_type {
                                PrivilegeType::Edit => bail!(
                                    "you are not allowed to {} in {}",
                                    privilege.to_string(),
                                    module.to_string()
                                ),
                                _ => return Ok(()),
                            },
                            _ => bail!(
                                "you are not allowed to {} in {}",
                                privilege.to_string(),
                                module.to_string()
                            ),
                        }
                    }
                }
            }
        }
        bail!(
            "you are not allowed to {} in {}",
            privilege.to_string(),
            module.to_string()
        )
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::BAD_REQUEST, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        AppError::Custom {
            status_code: status.as_u16(),
            error: error_message.to_owned(),
        }
        .into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| match e.reason() {
                TypedHeaderRejectionReason::Missing => AuthError::MissingCredentials,
                _ => AuthError::InvalidToken,
            })?;
        // Decode the user data
        let token_data =
            decode::<JwtClaims>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| AuthError::WrongCredentials)?;

        Ok(token_data.claims)
    }
}

#[macro_export]
macro_rules! cmp_company_id {
    ($e:expr, $c:expr) => {
        if $e.company_id == $c.company_id {
            CommonResponse::json_data($e)
        } else {
            Err(AppError::Custom {
                status_code: 400,
                error: "Invalid request".to_string(),
            })
        }
    };
}
