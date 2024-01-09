mod model;

use crate::controllers::remote_controller::RemotePortfolio;
use crate::error::gh_reqwestor_error::GhReqwestError;
use crate::gh_reqwestor;
use model::GithubRepository;
pub use model::Project;

const LIST_ORGANIZATION_REPOS_BASE_URL: &str = "https://api.github.com/orgs";
const RAW_CONTENT_BASE_URL: &str = "https://raw.githubusercontent.com/";

pub async fn list_projects_of_remote(
    remote: &RemotePortfolio,
    gh_auth_token: &str,
) -> Result<Vec<Project>, GhReqwestError> {
    let api_url = format!("{}/{}/repos", LIST_ORGANIZATION_REPOS_BASE_URL, remote.name);
    let mut projects: Vec<Project> = Vec::new();
    let repositories: Vec<GithubRepository> =
        gh_reqwestor::get::<Vec<GithubRepository>>(&api_url, gh_auth_token).await?;
    for repository in repositories.into_iter() {
        let mut new_project = Project::from(repository);
        new_project.set_description(
            get_project_description(&remote.name, &new_project.get_name(), gh_auth_token).await,
        );
        projects.push(new_project);
    }
    Ok(projects)
}

async fn get_project_description(
    remote_name: &String,
    project_name: &String,
    gh_auth_token: &str,
) -> Option<String> {
    let readme_content_url = format!(
        "{}/{}/{}/master/README.md",
        RAW_CONTENT_BASE_URL, remote_name, project_name
    );
    match gh_reqwestor::get_string(&readme_content_url, gh_auth_token).await {
        Err(_error) => {
            let readme_content_url = format!(
                "{}/{}/{}/master/readme.md",
                RAW_CONTENT_BASE_URL, remote_name, project_name
            );
            match gh_reqwestor::get_string(&readme_content_url, gh_auth_token).await {
                Err(_error) => None,
                Ok(content) => Some(content),
            }
        }
        Ok(content) => Some(content),
    }
}
