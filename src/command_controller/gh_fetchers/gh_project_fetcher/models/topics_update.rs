use serde::{Deserialize, Serialize};

/// To update topics on a repository
#[derive(Serialize, Deserialize, Debug)]
pub struct TopicsUpdate {
    pub names: Vec<String>,
}
