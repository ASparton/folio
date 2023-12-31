mod model;

use crate::error::gh_reqwestor_error::GhReqwestError;
use crate::gh_reqwestor;
use model::GithubOrganization;
pub use model::RemotePortfolio;

const LIST_REMOTES_URL: &str = "https://api.github.com/user/orgs";

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
pub async fn get_remotes(gh_auth_token: &str) -> Result<Vec<RemotePortfolio>, GhReqwestError> {
    let mut remotes: Vec<RemotePortfolio> = Vec::new();
    let user_orgs: Vec<GithubOrganization> =
        gh_reqwestor::get::<Vec<GithubOrganization>>(&LIST_REMOTES_URL, gh_auth_token).await?;
    for user_org in user_orgs.into_iter() {
        if user_org.is_portfolio() {
            remotes.push(RemotePortfolio::from(user_org))
        }
    }
    Ok(remotes)
}
