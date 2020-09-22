use crate::core::auth::hash;
use crate::core::database::PoolType;
use crate::core::errors::ApiError;
use crate::module::user::handler::{UserResponse, UsersResponse};
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}

/// Get all users
pub fn get_all(pool: &PoolType) -> Result<UsersResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let conn = pool.get()?;
    let all_users = users.load(&conn)?;

    Ok(all_users.into())
}

/// Find a user by the user's id or error out
pub fn find(pool: &PoolType, user_id: Uuid) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id, users};

    let not_found = format!("User {} not found", user_id);
    let conn = pool.get()?;
    let user = users
        .filter(id.eq(user_id.to_string()))
        .first::<User>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(user.into())
}

/// Find a user by the user's authentication information (email + password)
/// Return an Unauthorized error if it doesn't match
pub fn find_by_auth(
    pool: &PoolType,
    user_email: &str,
    user_password: &str,
) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{email, password, users};

    let conn = pool.get()?;
    let user = users
        .filter(email.eq(user_email.to_string()))
        .filter(password.eq(user_password.to_string()))
        .first::<User>(&conn)
        .map_err(|_| ApiError::Unauthorized("Invalid login".into()))?;
    Ok(user.into())
}

/// Create a new user
pub fn create(pool: &PoolType, new_user: &User) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let conn = pool.get()?;
    diesel::insert_into(users).values(new_user).execute(&conn)?;
    Ok(new_user.clone().into())
}

/// Update a user
pub fn update(pool: &PoolType, update_user: &UpdateUser) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id, users};

    let conn = pool.get()?;
    diesel::update(users)
        .filter(id.eq(update_user.id.clone()))
        .set(update_user)
        .execute(&conn)?;
    find(&pool, Uuid::parse_str(&update_user.id)?)
}

/// Delete a user
pub fn delete(pool: &PoolType, user_id: Uuid) -> Result<(), ApiError> {
    use crate::schema::users::dsl::{id, users};

    let conn = pool.get()?;
    diesel::delete(users)
        .filter(id.eq(user_id.to_string()))
        .execute(&conn)?;
    Ok(())
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: hash(&user.password),
            created_by: user.created_by,
            created_at: Utc::now().naive_utc(),
            updated_by: user.updated_by,
            updated_at: Utc::now().naive_utc(),
        }
    }
}
