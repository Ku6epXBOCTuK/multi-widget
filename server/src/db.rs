use chrono::{DateTime, Utc};
use shared::{Activity, ActivityId};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};
use std::str::FromStr;

pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_url = dotenv::var("DATABASE_URL").unwrap();

        let connection_options = SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connection_options)
            .await?;

        sqlx::migrate!("../migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn get_activities(&self) -> Result<Vec<Activity>, sqlx::Error> {
        let activities = sqlx::query_as(
            r#"
            select *,
                id AS "id: ActivityId",
                parent_id AS "parent_id: ActivityId",
                is_system AS "is_system: bool",
                created_at AS "created_at: DateTime<Utc>",
                updated_at AS "updated_at: DateTime<Utc>"
                from activities
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(activities)
    }
}
