#[allow(unused_imports)]
use chrono::{DateTime, Utc};
#[allow(unused_imports)]
use shared::{Activity, ActivityId};
use shared::{CreateActivityRequest, UpdateActivityRequest};
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

    pub async fn get_all_activities(&self) -> Result<Vec<Activity>, sqlx::Error> {
        let activities = sqlx::query_as(
            r#"
            select * from activities
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(activities)
    }

    // pub async fn get_activity_by_id(&self, id: ActivityId) -> Result<Activity, sqlx::Error> {
    //     sqlx::query_as("SELECT * FROM activities WHERE id = ?")
    //         .bind(id)
    //         .fetch_one(&self.pool)
    //         .await
    // }

    pub async fn update_activity(
        &self,
        update: UpdateActivityRequest,
    ) -> Result<Activity, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE activities
            SET parent_id = ?, activity_type = ?, title = ?, description = ?, status = ?, url = ?
            WHERE id = ?
            RETURNING *
            "#,
        )
        .bind(update.parent_id)
        .bind(update.activity_type)
        .bind(update.title)
        .bind(update.description)
        .bind(update.status)
        .bind(update.url)
        .bind(update.id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_activity(
        &self,
        create: CreateActivityRequest,
    ) -> Result<Activity, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO activities (parent_id, activity_type, title, description, status, url)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(create.parent_id)
        .bind(create.activity_type)
        .bind(create.title)
        .bind(create.description)
        .bind(create.status)
        .bind(create.url)
        .fetch_one(&self.pool)
        .await
    }
}
