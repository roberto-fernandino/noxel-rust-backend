use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::{
    models::{ConsumerData, OrganizerData, User, UserRole, UserRow},
    requests::{SignupAttendeeRequest, SignupOrganizerRequest},
};

/// Common accessor interface for different signup request payloads.
trait SignupRequestLike {
    fn full_name(&self) -> &str;
    fn password(&self) -> &str;
    fn email(&self) -> Option<&str>;
    fn gov_identification(&self) -> Option<i64>;
    fn birth_date(&self) -> Option<chrono::NaiveDate>;
}

impl SignupRequestLike for SignupOrganizerRequest {
    fn full_name(&self) -> &str {
        &self.full_name
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    fn gov_identification(&self) -> Option<i64> {
        self.gov_identification
    }

    fn birth_date(&self) -> Option<chrono::NaiveDate> {
        self.birth_date
    }
}

impl SignupRequestLike for SignupAttendeeRequest {
    fn full_name(&self) -> &str {
        &self.full_name
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    fn gov_identification(&self) -> Option<i64> {
        self.gov_identification
    }

    fn birth_date(&self) -> Option<chrono::NaiveDate> {
        Some(self.birth_date)
    }
}

/// Hash a password using Argon2id and return the PHC string.
fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default(); // Uses Argon2id by default
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

async fn insert_user<R: SignupRequestLike>(
    tx: &mut Transaction<'_, Postgres>,
    role: UserRole,
    req: &R,
) -> Result<User, sqlx::Error> {
    // Hash the password with Argon2id
    let password_hash = hash_password(req.password())
        .map_err(|e| sqlx::Error::Protocol(format!("Failed to hash password: {}", e)))?;

    let row: UserRow = sqlx::query_as(
        r#"INSERT INTO users (full_name, role, email, gov_identification, birth_date, password_hash)
           VALUES ($1, $2, $3, $4, $5, $6)
           RETURNING id, full_name, role, email, gov_identification, birth_date"#,
    )
    .bind(req.full_name())
    .bind(role.as_str())
    .bind(req.email())
    .bind(req.gov_identification())
    .bind(req.birth_date())
    .bind(&password_hash)
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.into_user())
}

pub async fn create_organizer_with_data(
    db: &PgPool,
    req: SignupOrganizerRequest,
) -> Result<(User, OrganizerData), sqlx::Error> {
    let mut tx = db.begin().await?;
    let user = insert_user(&mut tx, UserRole::Organizer, &req).await?;

    let org: OrganizerData = sqlx::query_as(
        r#"INSERT INTO organizer_data (user_id)
           VALUES ($1)
           RETURNING id, user_id, created_at"#,
    )
    .bind(user.id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok((user, org))
}

pub async fn create_attendee_with_data(
    db: &PgPool,
    req: SignupAttendeeRequest,
) -> Result<(User, ConsumerData), sqlx::Error> {
    let mut tx = db.begin().await?;
    let user = insert_user(&mut tx, UserRole::Attendee, &req).await?;

    let consumer: ConsumerData = sqlx::query_as(
        r#"INSERT INTO consumer_data (user_id)
           VALUES ($1)
           RETURNING id, user_id, created_at"#,
    )
    .bind(user.id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok((user, consumer))
}
