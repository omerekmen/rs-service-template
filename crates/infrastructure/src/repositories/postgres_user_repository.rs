use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use domain::{Email, User, UserRepository, UserStatus, Username};
use shared::{AppError, AppResult, UserId};

/// PostgreSQL implementation of UserRepository
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// Database model for users table
#[derive(sqlx::FromRow)]
struct UserRow {
    id: uuid::Uuid,
    username: String,
    email: String,
    full_name: Option<String>,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<UserRow> for User {
    type Error = AppError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        let username = Username::new(row.username)?;
        let email = Email::new(row.email)?;
        let status = match row.status.as_str() {
            "active" => UserStatus::Active,
            "inactive" => UserStatus::Inactive,
            "suspended" => UserStatus::Suspended,
            _ => {
                return Err(AppError::DatabaseError(format!(
                    "Invalid user status: {}",
                    row.status
                )));
            }
        };

        Ok(User::from_persistence(
            UserId::from_uuid(row.id),
            username,
            email,
            row.full_name,
            status,
            row.created_at,
            row.updated_at,
        ))
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> AppResult<()> {
        let status_str = match user.status() {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Suspended => "suspended",
        };

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, full_name, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(user.id().as_uuid())
        .bind(user.username().as_str())
        .bind(user.email().as_str())
        .bind(user.full_name())
        .bind(status_str)
        .bind(user.created_at())
        .bind(user.updated_at())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>> {
        let row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, username, email, full_name, status, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await?;

        row.map(|r| r.try_into()).transpose()
    }

    async fn find_by_username(&self, username: &Username) -> AppResult<Option<User>> {
        let row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, username, email, full_name, status, created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(username.as_str())
        .fetch_optional(&self.pool)
        .await?;

        row.map(|r| r.try_into()).transpose()
    }

    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
        let row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, username, email, full_name, status, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await?;

        row.map(|r| r.try_into()).transpose()
    }

    async fn update(&self, user: &User) -> AppResult<()> {
        let status_str = match user.status() {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Suspended => "suspended",
        };

        sqlx::query(
            r#"
            UPDATE users
            SET username = $2, email = $3, full_name = $4, status = $5, updated_at = $6
            WHERE id = $1
            "#,
        )
        .bind(user.id().as_uuid())
        .bind(user.username().as_str())
        .bind(user.email().as_str())
        .bind(user.full_name())
        .bind(status_str)
        .bind(user.updated_at())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: UserId) -> AppResult<()> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn username_exists(&self, username: &Username) -> AppResult<bool> {
        let result: Option<bool> = sqlx::query_scalar(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)
            "#,
        )
        .bind(username.as_str())
        .fetch_one(&self.pool)
        .await?;

        Ok(result.unwrap_or(false))
    }

    async fn email_exists(&self, email: &Email) -> AppResult<bool> {
        let result: Option<bool> = sqlx::query_scalar(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)
            "#,
        )
        .bind(email.as_str())
        .fetch_one(&self.pool)
        .await?;

        Ok(result.unwrap_or(false))
    }

    async fn list(&self, limit: i64, offset: i64) -> AppResult<Vec<User>> {
        let rows: Vec<UserRow> = sqlx::query_as(
            r#"
            SELECT id, username, email, full_name, status, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| row.try_into())
            .collect::<Result<Vec<_>, _>>()
    }

    async fn count(&self) -> AppResult<i64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }
}
