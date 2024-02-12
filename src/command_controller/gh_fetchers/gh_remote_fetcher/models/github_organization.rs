use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tabled::Tabled;

use crate::content_displayer::table_displayer::display_date;

/// Returned when getting the details of one organization of a user
#[derive(Serialize, Deserialize, Tabled)]
pub struct GithubOrganization {
    #[tabled(rename = "Name")]
    pub login: String,

    #[tabled(rename = "Projects count")]
    pub public_repos: u32,

    #[tabled(rename = "Created 🕰️", display_with = "display_date")]
    pub created_at: DateTime<Utc>,
}
