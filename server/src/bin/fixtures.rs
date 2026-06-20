use dotenv;
use serde::Deserialize;
use shared::{ActivityId, ActivityType};
use sqlx::SqlitePool;
use std::fs;

#[derive(Deserialize, Debug)]
struct FixtureItem {
    pub id: ActivityId,
    pub parent_id: Option<ActivityId>,
    pub activity_type: ActivityType,
    pub title: String,
    pub description: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = SqlitePool::connect(&db_url).await?;
    println!("Database connected ok");

    sqlx::query!("PRAGMA foreign_keys = OFF")
        .execute(&pool)
        .await?;
    sqlx::query!("DELETE FROM activities")
        .execute(&pool)
        .await?;
    sqlx::query!("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;
    println!("Database cleared");

    let json_content = fs::read_to_string("./server/src/bin/fixtures.json")?;
    let items: Vec<FixtureItem> = serde_json::from_str(&json_content)?;

    println!("File read ok");

    for item in items {
        sqlx::query!(
            r#"
            INSERT INTO activities (id, parent_id, activity_type, title, description)
            VALUES (?, ?, ?, ?, ?)
            "#,
            item.id as ActivityId,
            item.parent_id as Option<ActivityId>,
            item.activity_type as ActivityType,
            item.title,
            item.description
        )
        .execute(&pool)
        .await?;

        println!("Activity added: {}", item.title);
    }

    println!("Database fulfilled with fixtures");
    Ok(())
}
