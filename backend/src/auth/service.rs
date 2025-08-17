use bcrypt::verify;
#[cfg(test)]
use bcrypt::{DEFAULT_COST, hash};
use sqlx::SqlitePool;

use crate::errors::AppError;

use super::User;

pub async fn authenticate_user(
    db: &SqlitePool,
    email: &str,
    password: &str,
) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, name FROM users WHERE email = ?",
    )
    .bind(email)
    .fetch_optional(db)
    .await?;

    let user = user.ok_or_else(|| AppError::unauthorized())?;

    let password_hash: String = sqlx::query_scalar(
        "SELECT password_hash FROM users WHERE email = ?"
    )
    .bind(email)
    .fetch_one(db)
    .await?;

    if !verify(password, &password_hash)? {
        return Err(AppError::unauthorized());
    }

    Ok(user)
}

#[cfg(test)]
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|e| AppError::Internal(e.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "test123";
        let hash = hash_password(password).unwrap();
        assert!(verify(password, &hash).unwrap());
    }
}
