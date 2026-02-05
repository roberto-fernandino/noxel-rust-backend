use super::{
    models::{AttendeeData, OrganizerData, User, UserRole, UserRow},
    requests::{SignupAttendeeRequest, SignupOrganizerRequest},
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

/// Common accessor interface for different signup request payloads.
trait SignupRequestLike {
    fn full_name(&self) -> &str;
    fn password(&self) -> &str;
    fn email(&self) -> &str;
    fn gov_identification(&self) -> i64;
}

impl SignupRequestLike for SignupOrganizerRequest {
    fn full_name(&self) -> &str {
        &self.full_name
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn gov_identification(&self) -> i64 {
        self.gov_identification
    }
}

impl SignupRequestLike for SignupAttendeeRequest {
    fn full_name(&self) -> &str {
        &self.full_name
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn gov_identification(&self) -> i64 {
        self.gov_identification
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
        r#"INSERT INTO users (full_name, role, email, gov_identification, password_hash)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, full_name, role, email, gov_identification"#,
    )
    .bind(req.full_name())
    .bind(role.as_str())
    .bind(req.email())
    .bind(req.gov_identification())
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
) -> Result<(User, AttendeeData), sqlx::Error> {
    let mut tx = db.begin().await?;
    let user = insert_user(&mut tx, UserRole::Attendee, &req).await?;

    let consumer: AttendeeData = sqlx::query_as::<_, AttendeeData>(
        r#"INSERT INTO consumer_data (user_id, phone, birth_date)
           VALUES ($1, $2, $3)
           RETURNING id, user_id, phone, birth_date, created_at"#,
    )
    .bind(user.id)
    .bind(&req.phone)
    .bind(req.birth_date)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok((user, consumer))
}

impl AttendeeData {
    pub async fn get_data(pool: &PgPool, user_id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, AttendeeData>(r#"SELECT * FROM attendee_data WHERE user_id = $1"#)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| e)
    }
}

impl OrganizerData {
    pub async fn get_data(pool: &PgPool, user_id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, OrganizerData>(r#"SELECT * FROM organizer_data WHERE user_id = $1"#)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| e)
    }
}
