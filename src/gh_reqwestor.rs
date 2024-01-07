use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::error::gh_reqwestor_error::GhReqwestError;

pub async fn get<T: DeserializeOwned>(url: &str, gh_auth_token: &str) -> Result<T, GhReqwestError> {
    Ok(reqwest::Client::new()
        .get(url)
        .headers(get_gh_common_headers())
        .bearer_auth(gh_auth_token)
        .send()
        .await?
        .json::<T>()
        .await?)
}

const COMMON_HEADERS: [(&str, &str); 3] = [
    ("Accept", "application/vnd.github+json"),
    ("X-GitHub-Api-Version", "2022-11-28"),
    ("User-Agent", "folio-cli"),
];

/// Build a `reqwest::header::HeaderMap` with common headers required to use the GitHub API.
fn get_gh_common_headers() -> HeaderMap {
    let mut common_headers = HeaderMap::new();
    for (header_name, header_value) in COMMON_HEADERS.into_iter() {
        common_headers.append(header_name, header_value.parse().unwrap());
    }
    common_headers
}
