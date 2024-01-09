mod model;

use crate::controllers::remote_controller::RemotePortfolio;
use crate::error::gh_reqwestor_error::GhReqwestError;
use crate::gh_reqwestor;
use model::GithubRepository;
pub use model::Project;

pub use crate::controllers::project_controller::model::GithubContent;

const LIST_ORGANIZATION_REPOS_BASE_URL: &str = "https://api.github.com/orgs";
const REPOS_BASE_URL: &str = "https://api.github.com/repos";
const RAW_CONTENT_BASE_URL: &str = "https://raw.githubusercontent.com";

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
        new_project.set_images_url(
            get_images_url(&remote.name, &new_project.get_name(), gh_auth_token).await,
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

async fn get_images_url(
    remote_name: &String,
    project_name: &String,
    gh_auth_token: &str,
) -> Vec<String> {
    let mut images_url: Vec<String> = Vec::new();
    let images_folder_url = format!(
        "{}/{}/{}/contents/images",
        REPOS_BASE_URL, remote_name, project_name
    );
    match gh_reqwestor::get::<Vec<GithubContent>>(&images_folder_url, gh_auth_token).await {
        Err(_error) => return images_url,
        Ok(folder_content) => {
            for file in folder_content {
                images_url.push(file.download_url);
            }
            images_url
        }
    }
}
