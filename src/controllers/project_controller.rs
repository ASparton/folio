mod model;

use crate::controllers::remote_controller::RemotePortfolio;
use crate::error::GhReqwestError;
use crate::gh_reqwestor;
use model::GithubRepository;
pub use model::Project;

const LIST_ORGANIZATION_REPOS_BASE_URL: &str = "https://api.github.com/orgs";

pub async fn list_projects_of_remote(
    remote: &RemotePortfolio,
    gh_auth_token: &str,
) -> Result<Vec<Project>, GhReqwestError> {
    let api_url = format!("{}/{}/repos", LIST_ORGANIZATION_REPOS_BASE_URL, remote.name);
    let mut projects: Vec<Project> = Vec::new();
    let repositories: Vec<GithubRepository> =
        gh_reqwestor::get::<Vec<GithubRepository>>(&api_url, gh_auth_token).await?;
    for repository in repositories.into_iter() {
        projects.push(Project::from(repository));
    }
    Ok(projects)
}
