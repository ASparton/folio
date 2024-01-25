use chrono::{DateTime, Utc};
use serde::Deserialize;
use tabled::Tabled;

/// Returned when getting the details of one organization of a user
#[derive(Deserialize, Tabled)]
pub struct GithubOrganization {
    #[tabled(rename = "Name")]
    pub login: String,

    #[tabled(rename = "Projects count")]
    pub public_repos: u32,

    #[tabled(rename = "Created 🕰️", display_with = "display_date")]
    pub created_at: DateTime<Utc>,
}

fn display_date(date: &DateTime<Utc>) -> String {
    format!("{}", date.date_naive())
}
