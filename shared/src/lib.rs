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

#[derive(Debug, Type, Serialize, Deserialize, Display, SqlxType, EnumString)]
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

#[derive(Debug, Type, Serialize, Deserialize, Display, SqlxType, EnumString)]
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

#[derive(Debug, Type, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Type, Serialize, Deserialize)]
pub enum WsEvent {
    AddActivity(Activity),
    UpdateActivity(Activity),
    Delete(ActivityId),
}
