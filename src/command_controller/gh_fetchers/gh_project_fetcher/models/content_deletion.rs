use serde::Serialize;

#[derive(Serialize)]
pub struct ContentDeletion {
    pub message: String,
    pub sha: String,
}
