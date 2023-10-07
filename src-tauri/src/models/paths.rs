use std::path::Path;

use failure::Error;
use ormlite::{
    sqlite::Sqlite,
    types::{
        chrono::{DateTime, Utc},
        Json,
    },
    Model, Pool, model::Insertable,
};
use serde::Serialize;

#[derive(Debug, Model, Serialize)]
#[ormlite(insertable = InsertPaths)]
pub struct Paths {
    pub id: i32,
    #[ormlite(default)]
    pub created_at: DateTime<Utc>,
    #[ormlite(default)]
    pub updated_at: DateTime<Utc>,
    #[ormlite(default)]
    pub version: i32,

    pub path: String,         // 文件的路径
    pub name: Option<String>, // 文件或文件夹名
    pub is_dir: bool,
    pub size: i64, // 文件大小
    pub file_created_at: DateTime<Utc>,
    pub last_modified_at: DateTime<Utc>,

    #[ormlite(default_value = "serde_json::json!([])")]
    pub tags: Json<Vec<i32>>,
}

pub async fn insert_paths(pool: &Pool<Sqlite>, p: &Path)-> Result<(), Error> {
    let ip: InsertPaths = p.into();
    ip.insert(pool).await?;
    Ok(())
}

impl From<&Path> for InsertPaths {
    fn from(pb: &Path) -> Self {
        let meta = pb.metadata().unwrap();
        Self {
            path: pb.to_string_lossy().to_string(),
            name: pb.file_name().map(|s| s.to_string_lossy().to_string()),
            is_dir: pb.is_dir(),
            size: meta.len() as i64,
            file_created_at: meta.created().unwrap().into(),
            last_modified_at: meta.modified().unwrap().into(),
        }
    }
}
