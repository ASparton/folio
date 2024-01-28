use serde::Serialize;

/// Github repository creation fields
#[derive(Serialize)]
pub struct GithubRepositoryCreation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    pub description: Option<String>,
}
