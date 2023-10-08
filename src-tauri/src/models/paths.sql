DROP TABLE IF EXISTS `paths`;
CREATE TABLE `paths` (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    version INTEGER,
    path TEXT,
    name TEXT,
    is_dir boolean,
    size INTEGER,
    file_created_at TIMESTAMP,
    last_modified_at TIMESTAMP,
    tags JSON
);