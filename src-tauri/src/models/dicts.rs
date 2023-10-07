use ormlite::{types::chrono::{DateTime, Utc}, Model};
use serde::Serialize;

#[derive(Debug, Model, Serialize)]
#[ormlite(insertable=InsertDict)]
pub struct Dict {
    pub id: i32,
    #[ormlite(default)]
    pub created_at: DateTime<Utc>,
    #[ormlite(default)]
    pub updated_at: DateTime<Utc>,
    #[ormlite(default)]
    pub version: i32,
    pub name: String,
    pub category: String,
    #[ormlite(default)]
    pub father_id: i32,
    #[ormlite(default)]
    pub pinyin: Option<String>,
}
