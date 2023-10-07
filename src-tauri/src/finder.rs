use std::{
    fs::{create_dir_all, File},
    path::{Path, PathBuf},
};

use crate::utils::make_sure_file_exists;
use failure::Error;
use ormlite::{
    sqlite::{Sqlite, SqlitePoolOptions},
    Pool, PoolOptions,
};

const STORE_PATH: &'static str = "./sqlite.finder";
// const INDEX_PATH: &'static str = "index.finder";
// const PATH_FIELD: &'static str = "path";

#[derive(Debug)]
pub struct Finder {
    pub pool: Pool<Sqlite>,
}

impl Finder {
    pub async fn new() -> Result<Self, Error> {
        make_sure_file_exists(STORE_PATH);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(STORE_PATH)
            .await?;

        Ok(Self {
            pool,
        })
    }
}
