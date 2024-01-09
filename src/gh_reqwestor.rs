use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};

pub async fn get<T: DeserializeOwned>(url: &str, gh_auth_token: &str) -> Result<T, reqwest::Error> {
    let response = match reqwest::Client::new()
        .get(url)
        .headers(get_gh_common_headers())
        .bearer_auth(gh_auth_token)
        .send()
        .await
    {
        Err(error) => return Err(error),
        Ok(res) => match res.error_for_status() {
            Err(error) => return Err(error),
            Ok(res) => res,
        },
    };
    Ok(response.json::<T>().await?)
}

pub async fn get_string(url: &str, gh_auth_token: &str) -> Result<String, reqwest::Error> {
    let response = match reqwest::Client::new()
        .get(url)
        .headers(get_gh_common_headers())
        .bearer_auth(gh_auth_token)
        .send()
        .await
    {
        Err(error) => return Err(error),
        Ok(res) => match res.error_for_status() {
            Err(error) => return Err(error),
            Ok(res) => res,
        },
    };
    Ok(response.text().await?)
}

pub async fn post<I: Serialize, T: DeserializeOwned>(
    url: &str,
    body: &I,
    gh_auth_token: &str,
) -> Result<T, reqwest::Error> {
    let response = match reqwest::Client::new()
        .post(url)
        .headers(get_gh_common_headers())
        .bearer_auth(gh_auth_token)
        .json::<I>(&body)
        .send()
        .await
    {
        Err(error) => return Err(error),
        Ok(res) => match res.error_for_status() {
            Err(error) => return Err(error),
            Ok(res) => res,
        },
    };
    Ok(response.json::<T>().await?)
}

pub async fn put<I: Serialize, T: DeserializeOwned>(
    url: &str,
    body: &I,
    gh_auth_token: &str,
) -> Result<T, reqwest::Error> {
    let response = match reqwest::Client::new()
        .put(url)
        .headers(get_gh_common_headers())
        .bearer_auth(gh_auth_token)
        .json::<I>(&body)
        .send()
        .await
    {
        Err(error) => return Err(error),
        Ok(res) => match res.error_for_status() {
            Err(error) => return Err(error),
            Ok(res) => res,
        },
    };
    Ok(response.json::<T>().await?)
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
