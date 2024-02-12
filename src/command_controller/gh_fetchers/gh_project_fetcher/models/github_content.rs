use serde::Deserialize;

/// Github content (file) fields
#[derive(Deserialize)]
pub struct GithubContent {
    pub download_url: String,
}
