use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Returned when getting the details of one organization of a user
#[derive(Deserialize)]
pub struct GithubOrganization {
    pub login: String,
    pub public_repos: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}