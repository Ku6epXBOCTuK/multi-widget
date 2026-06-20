#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::{
    Type as SqlxType,
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};
use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Debug, Type, Serialize, Deserialize, Display, SqlxType, EnumString, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
#[sqlx(type_name = "TEXT", rename_all = "kebab-case")]
pub enum ActivityStatus {
    Pending,
    InProgress,
    Done,
}

impl From<String> for ActivityStatus {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap_or(ActivityStatus::Pending)
    }
}

#[derive(Debug, Type, Serialize, Deserialize, Display, SqlxType, EnumString, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
#[sqlx(type_name = "TEXT", rename_all = "kebab-case")]
pub enum ActivityType {
    Project,
    Stage,
    Task,
    SubTask,
}

impl From<String> for ActivityType {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap_or(ActivityType::Task)
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SqlxType, Type, Serialize, Deserialize,
)]
#[sqlx(transparent)]
pub struct ActivityId(i32);

#[derive(Debug, Type, Serialize, Deserialize, FromRow, Clone)]
pub struct Activity {
    pub id: ActivityId,
    pub parent_id: Option<ActivityId>,
    pub activity_type: ActivityType,
    pub title: String,
    pub description: Option<String>,
    pub status: ActivityStatus,
    pub url: Option<String>,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Type, Serialize, Deserialize, Clone)]
pub struct GetActivityRequest {
    pub id: ActivityId,
}

#[derive(Debug, Type, Serialize, Deserialize, Clone)]
pub struct UpdateActivityRequest {
    pub id: ActivityId,
    pub parent_id: Option<ActivityId>,
    pub activity_type: ActivityType,
    pub title: String,
    pub description: Option<String>,
    pub status: ActivityStatus,
    pub url: Option<String>,
}

#[derive(Debug, Type, Serialize, Deserialize, Clone)]
pub struct CreateActivityRequest {
    pub parent_id: Option<ActivityId>,
    pub activity_type: ActivityType,
    pub title: String,
    pub description: Option<String>,
    pub status: ActivityStatus,
    pub url: Option<String>,
}

impl Activity {
    pub fn create_from(create: CreateActivityRequest) -> Self {
        Self {
            id: ActivityId(12345),
            parent_id: create.parent_id,
            activity_type: create.activity_type,
            title: create.title,
            description: create.description,
            status: create.status,
            url: create.url,
            is_system: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    pub fn update_from(&mut self, update: UpdateActivityRequest) {
        self.parent_id = update.parent_id;
        self.activity_type = update.activity_type;
        self.title = update.title;
        self.description = update.description;
        self.status = update.status;
        self.url = update.url;
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Type, Serialize, Deserialize)]
pub enum WsEvent {
    AddActivity(Activity),
    UpdateActivity(Activity),
    Delete(ActivityId),
}
