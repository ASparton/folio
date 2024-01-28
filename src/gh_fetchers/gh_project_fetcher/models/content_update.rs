use serde::Serialize;

#[derive(Serialize)]
pub struct ContentUpdate {
    pub message: String,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sha: Option<String>,
}
