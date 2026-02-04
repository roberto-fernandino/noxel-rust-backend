use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::{
    models::{ConsumerData, OrganizerData, User, UserRole, UserRow},
    requests::SignupRequest,
};

// NOTE: list/get/create stubs can be implemented later.

pub async fn list_users(_db: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    Ok(vec![])
}

pub async fn get_user_by_id(_db: &PgPool, _id: Uuid) -> Result<Option<User>, sqlx::Error> {
    Ok(None)
}

async fn insert_user(
    tx: &mut Transaction<'_, Postgres>,
    role: UserRole,
    req: &SignupRequest,
) -> Result<User, sqlx::Error> {
    let row: UserRow = sqlx::query_as(
        r#"INSERT INTO users (full_name, role, email, gov_identification, birth_date)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, full_name, role, email, gov_identification, birth_date"#,
    )
    .bind(&req.full_name)
    .bind(role.as_str())
    .bind(&req.email)
    .bind(req.gov_identification)
    .bind(req.birth_date)
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.into_user())
}

pub async fn create_organizer_with_data(
    db: &PgPool,
    req: SignupRequest,
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
    req: SignupRequest,
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
