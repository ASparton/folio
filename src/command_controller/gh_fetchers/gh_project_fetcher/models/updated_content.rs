use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdatedContent {
    content: UpdatedContentInner,
}

#[derive(Deserialize, Debug)]
struct UpdatedContentInner {
    name: String,
}