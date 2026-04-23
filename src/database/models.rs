use chrono::{Date, DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Repository {
    pub id: i64,
    pub url: String,
    pub language: String,
    pub stars: i32,
    pub added_at: DateTime<Utc>,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub id: i64,
    pub repository_id: i64,
    pub original_code: String,
    pub normalize_code: String,
    pub ast_hash: String,
    pub start_line: u32,
    pub end_line: u32,
    pub file_path: String,
    pub snippet_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Fingerprint {
    pub id: i64,
    pub snippet_id: i64,
    pub hash_type: String,
    pub hash_value: String,
    pub position: u32,
}

#[derive(Debug, Clone)]
pub struct ScanSession {
    pub id: i64,
    pub status: String,
    pub source_path: String,
    pub language: String,
    pub min_similarity: f64,
    pub total_snippets: u32,
    pub processed_snippets: u32,
    pub matches_found: u32,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: i64,
    pub query_snippet_id: i64,
    pub matched_snippet_id: i64,
    pub similarity_score: f64,
    pub matched_lines: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}
