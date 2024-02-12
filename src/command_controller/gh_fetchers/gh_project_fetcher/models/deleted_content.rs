use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeletedContent {
    content: Option<String>,
}
