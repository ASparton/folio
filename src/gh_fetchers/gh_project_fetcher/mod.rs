mod model;

use crate::error::gh_reqwestor_error::GhReqwestError;
use crate::gh_reqwestor;
use model::GithubRepository;
pub use model::Project;

pub use crate::gh_fetchers::gh_project_fetcher::model::GithubContent;

const LIST_ORGANIZATION_REPOS_BASE_URL: &str = "https://api.github.com/orgs";
const REPOS_BASE_URL: &str = "https://api.github.com/repos";
const RAW_CONTENT_BASE_URL: &str = "https://raw.githubusercontent.com";

pub async fn list_projects_of_remote(
    remote_name: &String,
    gh_auth_token: &str,
) -> Result<Vec<Project>, GhReqwestError> {
    let api_url = format!("{}/{}/repos", LIST_ORGANIZATION_REPOS_BASE_URL, remote_name);
    let mut projects: Vec<Project> = Vec::new();
    let repositories: Vec<GithubRepository> =
        gh_reqwestor::get::<Vec<GithubRepository>>(&api_url, gh_auth_token).await?;
    for repository in repositories.into_iter() {
        projects.push(build_project_from_repository(repository, remote_name, gh_auth_token).await);
    }
    Ok(projects)
}

pub async fn get_project_of_remote(
    remote_name: &String,
    project_name: &String,
    gh_auth_token: &str,
) -> Result<Project, GhReqwestError> {
    let api_url = format!("{}/{}/{}", REPOS_BASE_URL, remote_name, project_name);
    let repository = gh_reqwestor::get::<GithubRepository>(&api_url, gh_auth_token).await?;
    Ok(Project::from(
        build_project_from_repository(repository, remote_name, gh_auth_token).await,
    ))
}

async fn build_project_from_repository(
    repository: GithubRepository,
    remote_name: &String,
    gh_auth_token: &str,
) -> Project {
    let mut project = Project::from(repository);
    project.set_description(
        get_project_description(remote_name, &project.get_name(), gh_auth_token).await,
    );
    project.set_images_url(get_images_url(remote_name, &project.get_name(), gh_auth_token).await);
    project
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
