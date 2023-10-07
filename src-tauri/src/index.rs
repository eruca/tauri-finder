use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use failure::Error;
use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{Document, Schema, STORED, TEXT},
    Index, IndexWriter, ReloadPolicy,
};
use tokio::{fs::DirEntry, sync::mpsc::Receiver};

struct TantivyIndex {}

impl TantivyIndex {
    async fn process(&self, writer: IndexWriter, mut receiver: &mut Receiver<PathBuf>) {
        for path in receiver.recv().await {}
    }
}

#[cfg(windows)]
pub fn normalize_path(path: &Path) -> Cow<Path> {
    match path.to_str() {
        Some(p) => {
            if p.contains("\\") {
                Cow::Owned(p.replace("\\", "/").into())
            } else {
                Cow::Borrowed(path)
            }
        }
        None => panic!("{:?} not valid utf-8 path", path),
    }
}

#[cfg(unix)]
pub fn normalize_path(path: &Path) -> Cow<Path> {
    Cow::Borrowed(path)
}
