# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


```sql
SELECT
	id
	, path
	, is_dir
	, size
	, (LENGTH(path) - LENGTH(REPLACE(path, '/', ''))) AS layers
-- 	, CASE WHEN path REGEXP '\bapple\b' THEN 1 ELSE 0 END AS matched
	, CASE WHEN path REGEXP '^/Users/nick/' THEN 1 ELSE 0 END as ishome
FROM paths
WHERE path REGEXP '\.pptx?$'
ORDER BY ishome DESC, layers
```