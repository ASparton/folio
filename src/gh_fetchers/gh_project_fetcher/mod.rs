mod models;

use crate::error::gh_reqwestor_error::GhReqwestError;
pub use crate::gh_fetchers::gh_project_fetcher::models::project::Project;
use crate::gh_fetchers::gh_project_fetcher::models::{
    content_deletion::ContentDeletion, content_update::ContentUpdate,
    deleted_content::DeletedContent, github_content::GithubContent,
    github_repository::GithubRepository, github_repository_creation::GithubRepositoryCreation,
    topics_update::TopicsUpdate, updated_content::UpdatedContent,
};
use crate::gh_reqwestor;

const ORGANIZATION_REPOS_BASE_URL: &str = "https://api.github.com/orgs";
const REPOS_BASE_URL: &str = "https://api.github.com/repos";
const RAW_CONTENT_BASE_URL: &str = "https://raw.githubusercontent.com";

pub async fn list_projects_of_remote(
    remote_name: &String,
    gh_auth_token: &str,
) -> Result<Vec<Project>, GhReqwestError> {
    let api_url = format!("{}/{}/repos", ORGANIZATION_REPOS_BASE_URL, remote_name);
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

pub async fn create_remote_project(
    remote_name: &String,
    project_values: &GithubRepositoryCreation,
    gh_auth_token: &str,
) -> Result<Project, GhReqwestError> {
    let api_url = format!("{}/{}/repos", ORGANIZATION_REPOS_BASE_URL, remote_name);
    let created_repo = gh_reqwestor::post::<GithubRepositoryCreation, GithubRepository>(
        api_url.as_str(),
        project_values,
        gh_auth_token,
    )
    .await?;
    Ok(Project::from(
        build_project_from_repository(created_repo, remote_name, gh_auth_token).await,
    ))
}

pub async fn update_remote_project(
    remote_name: &String,
    project_name: &String,
    project_values: &GithubRepositoryCreation,
    gh_auth_token: &str,
) -> Result<Project, GhReqwestError> {
    let api_url = format!("{}/{}/{}", REPOS_BASE_URL, remote_name, project_name);
    let updated_repo = gh_reqwestor::patch::<GithubRepositoryCreation, GithubRepository>(
        api_url.as_str(),
        project_values,
        gh_auth_token,
    )
    .await?;
    Ok(Project::from(
        build_project_from_repository(updated_repo, remote_name, gh_auth_token).await,
    ))
}

pub async fn update_project_topics(
    remote_name: &String,
    project_name: &String,
    topics: &Vec<String>,
    gh_auth_token: &str,
) -> Result<TopicsUpdate, GhReqwestError> {
    let api_url = format!("{}/{}/{}/topics", REPOS_BASE_URL, remote_name, project_name);
    let body = TopicsUpdate {
        names: topics.clone(),
    };
    let updated_topics =
        gh_reqwestor::put::<TopicsUpdate, TopicsUpdate>(&api_url, &body, gh_auth_token).await?;
    Ok(updated_topics)
}

pub async fn update_project_content(
    remote_name: &String,
    project_name: &String,
    content_path: &String,
    content_base64: &String,
    commit_message: &String,
    file_sha: &Option<String>,
    gh_auth_token: &str,
) -> Result<UpdatedContent, GhReqwestError> {
    let api_url = format!(
        "{}/{}/{}/contents/{}",
        REPOS_BASE_URL, remote_name, project_name, content_path
    );
    let body = ContentUpdate {
        content: content_base64.clone(),
        message: commit_message.clone(),
        sha: file_sha.clone(),
    };
    Ok(gh_reqwestor::put::<ContentUpdate, UpdatedContent>(&api_url, &body, gh_auth_token).await?)
}

pub async fn delete_project_file(
    remote_name: &String,
    project_name: &String,
    file_path: &String,
    commit_message: &String,
    file_sha: &String,
    gh_auth_token: &str,
) -> Result<DeletedContent, GhReqwestError> {
    let api_url = format!(
        "{}/{}/{}/contents/{}",
        REPOS_BASE_URL, remote_name, project_name, file_path
    );
    let body = ContentDeletion {
        message: commit_message.clone(),
        sha: file_sha.clone(),
    };
    match gh_reqwestor::delete::<ContentDeletion, DeletedContent>(
        &api_url,
        Some(body),
        gh_auth_token,
    )
    .await
    {
        Err(error) => {
            println!("{:?}", error);
            return Err(GhReqwestError::from(error));
        }
        Ok(t) => Ok(t),
    }
}

pub async fn delete_project(
    remote_name: &String,
    project_name: &String,
    gh_auth_token: &str,
) -> Result<bool, GhReqwestError> {
    let api_url = format!("{}/{}/{}", REPOS_BASE_URL, remote_name, project_name);
    Ok(gh_reqwestor::plain_delete(&api_url, gh_auth_token).await?)
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
