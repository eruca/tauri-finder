use std::{
    fs::{create_dir_all, File},
    path::{Path, PathBuf},
};

use failure::Error;
use kv::{Config, Store};
use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    query::QueryParser,
    schema::{Document, Field, Schema, STORED, TEXT},
    Index, IndexReader, IndexWriter, ReloadPolicy,
};
use tantivy_jieba::JiebaTokenizer;

use crate::index::TantivyIndexReader;
use crate::utils::make_sure_file_exists;

const STORE_PATH: &'static str = "store.finder";
const INDEX_PATH: &'static str = "index.finder";
const PATH_FIELD: &'static str = "path";

#[derive(Debug)]
pub struct Finder {
    store: Store,
    index: Index,
    schema: Schema,
    fields: Vec<Field>,
}

impl Finder {
    pub fn new() -> Self {
        make_sure_file_exists(Path::new(STORE_PATH));
        make_sure_file_exists(Path::new(INDEX_PATH));

        let cfg = Config::new(STORE_PATH);
        let store = Store::new(cfg).unwrap();

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field(PATH_FIELD, TEXT | STORED);
        let schema = schema_builder.build();

        let path_field = schema.get_field(PATH_FIELD).unwrap();

        let dir = MmapDirectory::open(INDEX_PATH).unwrap();
        let index = Index::open_or_create(dir, schema.clone()).unwrap();

        Self {
            store,
            index,
            schema,
            fields: vec![path_field],
        }
    }

    pub fn build_index_reader(&self) -> Result<TantivyIndexReader, Error> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;
        
        Ok(TantivyIndexReader {
            reader,
            query_parser: QueryParser::for_index(&self.index, self.fields.clone()),
            tokenizer: JiebaTokenizer {},
        })
    }

    pub fn query(&self) {}
}
