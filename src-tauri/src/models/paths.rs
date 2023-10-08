use std::{path::Path, time::SystemTime};

use failure::Error;
use ormlite::{
    model::Insertable,
    sqlite::Sqlite,
    types::{
        chrono::{DateTime, Local},
        Json,
    },
    Executor, Model, Pool,
};
use serde::Serialize;

#[derive(Debug, Model, Serialize)]
#[ormlite(insertable = InsertPaths)]
pub struct Paths {
    pub id: i32,
    #[ormlite(default_value = "now()")]
    pub created_at: DateTime<Local>,
    #[ormlite(default_value = "now()")]
    pub updated_at: DateTime<Local>,
    #[ormlite(default_value = "1")]
    pub version: i32,

    pub path: String,         // 文件的路径
    pub name: Option<String>, // 文件或文件夹名
    pub is_dir: bool,
    pub size: i64, // 文件大小
    pub file_created_at: Option<DateTime<Local>>,
    pub last_modified_at: DateTime<Local>,

    #[ormlite(default)]
    pub tags: Option<Json<Vec<i32>>>,
}

fn now() -> DateTime<Local> {
    Local::now()
}

pub async fn init_table(pool: &Pool<Sqlite>) -> Result<(), Error> {
    pool.execute(include_str!("./paths.sql")).await?;
    Ok(())
}

pub async fn insert_paths(pool: &Pool<Sqlite>, p: &Path) -> Result<(), Error> {
    let ip: InsertPaths = p.try_into()?;
    ip.insert(pool).await?;
    Ok(())
}

impl TryFrom<&Path> for InsertPaths {
    type Error = Error;
    fn try_from(pb: &Path) -> Result<Self, Error> {
        let meta = pb.metadata()?;
        let created_at: Option<DateTime<Local>> = match meta.created().ok() {
            Some(st) if st != std::time::UNIX_EPOCH => Some(SystemTime::from(st).into()),
            _ => None,
        };

        Ok(Self {
            path: pb.to_string_lossy().to_string(),
            name: pb.file_name().map(|s| s.to_string_lossy().to_string()),
            is_dir: pb.is_dir(),
            size: meta.len() as i64,
            file_created_at: created_at,
            last_modified_at: meta.modified().unwrap().into(),
        })
    }
}
