use crate::core::auth::{create_jwt, hash, PrivateClaim};
use crate::core::database::PoolType;
use crate::core::errors::ApiError;
use crate::helper::respond::{respond_json, respond_ok};
use crate::module::user::handler::UserResponse;
use crate::module::user::model::find_by_auth;
use crate::core::validate::validate;
use actix_identity::Identity;
use actix_web::web::{block, Data, HttpResponse, Json};
use serde::Serialize;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

/// Login a user
/// Create and remember their JWT
pub async fn login(
    id: Identity,
    pool: Data<PoolType>,
    params: Json<LoginRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;

    // Validate that the email + hashed password matches
    let hashed = hash(&params.password);
    let user = block(move || find_by_auth(&pool, &params.email, &hashed)).await?;

    // Create a JWT
    let private_claim = PrivateClaim::new(user.id, user.email.clone());
    let jwt = create_jwt(private_claim)?;

    // Remember the token
    id.remember(jwt);
    respond_json(user.into())
}

/// Logout a user
/// Forget their user_id
pub async fn logout(id: Identity) -> Result<HttpResponse, ApiError> {
    id.forget();
    respond_ok()
}
