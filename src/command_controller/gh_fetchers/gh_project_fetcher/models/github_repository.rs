use serde::Deserialize;

/// GitHub reopsitory fields
#[derive(Debug, Deserialize)]
pub struct GithubRepository {
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub topics: Vec<String>,
}
