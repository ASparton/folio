use reqwest::Request;
use serde::de::DeserializeOwned;

pub async fn get<T: DeserializeOwned>(url: &str, gh_auth_token: &str) -> Result<T, dyn std::error::Error> {
    reqwest::Client::new()
        .get(url)
        .bearer_auth(gh_auth_token)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send().await?
        .json::<T>().await?
}
