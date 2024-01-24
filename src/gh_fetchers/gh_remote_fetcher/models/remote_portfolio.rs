use chrono::{DateTime, Utc};
use std::fmt::Display;

use crate::gh_fetchers::gh_remote_fetcher::models::github_organization::GithubOrganization;

/// Describes a remote organization used to store portfolio projects.
#[derive(Debug)]
pub struct RemotePortfolio {
    name: String,
    projects_count: u32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<GithubOrganization> for RemotePortfolio {
    fn from(value: GithubOrganization) -> Self {
        RemotePortfolio {
            name: value.login,
            projects_count: value.public_repos,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl Display for RemotePortfolio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} projects, created: {}, last updated: {}",
            self.name, self.projects_count, self.created_at, self.updated_at
        )
    }
}
