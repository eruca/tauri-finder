use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use failure::Error;
use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{Document, Field, Schema, STORED, TEXT},
    Index, IndexReader, IndexWriter, ReloadPolicy,
};
use tantivy_jieba::JiebaTokenizer;
use tokio::{fs::DirEntry, sync::mpsc::Receiver};

use crate::finder::Finder;

pub struct TantivyIndexReader {
    pub reader: IndexReader,
    pub query_parser: QueryParser,
    pub tokenizer: JiebaTokenizer,
}

struct TantivyIndexWriter {
    pub writer: IndexWriter,
    pub path_field: Field,
    pub tokenizer: JiebaTokenizer,
}
