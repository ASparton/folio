mod models;

use crate::command_controller::gh_fetchers::gh_reqwestor;
use crate::error::gh_reqwestor_error::GhReqwestError;

use crate::command_controller::gh_fetchers::gh_remote_fetcher::models::{
    github_organization::GithubOrganization,
    github_organization_list_item::GithubOrganizationListItem,
};

const LIST_REMOTES_URL: &str = "https://api.github.com/user/orgs";
const REMOTE_DETAILS_URL: &str = "https://api.github.com/orgs";

/// Get the remote portfolios of the current user.
///
/// # Example
///
/// ```
/// match get_remotes("gh_token").await {
///     Ok(remotes) => println!("Remote portfolios: {:?}", remotes),
///     Err(err) ) => println!("Could not fetch remote portfolios: {:?}", err),
/// };
/// ```
pub async fn get_remotes(gh_auth_token: &str) -> Result<Vec<GithubOrganization>, GhReqwestError> {
    let mut remotes: Vec<GithubOrganization> = Vec::new();
    let user_orgs: Vec<GithubOrganizationListItem> =
        gh_reqwestor::get::<Vec<GithubOrganizationListItem>>(&LIST_REMOTES_URL, gh_auth_token)
            .await?;
    for user_org in user_orgs.into_iter() {
        if user_org.is_portfolio() {
            remotes.push(get_remote_from_gh_org_login(&user_org.login, gh_auth_token).await?)
        }
    }
    Ok(remotes)
}

pub async fn get_remotes_name(gh_auth_token: &str) -> Result<Vec<String>, GhReqwestError> {
    let mut remotes_name: Vec<String> = Vec::new();
    let user_orgs: Vec<GithubOrganizationListItem> =
        gh_reqwestor::get::<Vec<GithubOrganizationListItem>>(&LIST_REMOTES_URL, gh_auth_token)
            .await?;
    for user_org in user_orgs.into_iter() {
        if user_org.is_portfolio() {
            remotes_name.push(user_org.login);
        }
    }
    Ok(remotes_name)
}

/// Fetch the needed organization information related to the given login and
/// build a RemotePortfolio object from them.
///
/// # Example
///
/// ```
/// match get_remote_from_gh_org_login("gh_token").await {
///     Ok(remotes) => println!("Remote portfolio: {:?}", remote),
///     Err(err) ) => println!("Could not fetch remote portfolio: {:?}", err),
/// };
/// ```
async fn get_remote_from_gh_org_login(
    remote_login: &str,
    gh_auth_token: &str,
) -> Result<GithubOrganization, GhReqwestError> {
    let remote_details_url = format!("{}/{}", REMOTE_DETAILS_URL, remote_login);
    let organization =
        gh_reqwestor::get::<GithubOrganization>(&remote_details_url, gh_auth_token).await?;
    Ok(GithubOrganization::from(organization))
}
