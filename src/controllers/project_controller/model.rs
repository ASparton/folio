use std::fmt::Display;

use serde::Deserialize;

/// Github content (file) fields
#[derive(Deserialize)]
pub struct GithubContent {
    pub download_url: String,
}

/// GitHub reopsitory fields
#[derive(Deserialize)]
pub struct GithubRepository {
    name: String,
    full_name: String,
    html_url: String,
    description: String,
    stargazers_count: u32,
    watchers_count: u32,
    topics: Vec<String>,
}

/// Describes a portfolio project.
#[derive(Debug)]
pub struct Project {
    name: String,
    full_name: String,
    formatted_name: String,
    url: String,
    teaser: String,
    description: Option<String>,
    cover_url: String,
    topics: Vec<String>,
    stargazers_count: u32,
    watchers_count: u32,
    images_url: Vec<String>,
}

impl Project {
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_images_url(&mut self, images_url: Vec<String>) {
        self.images_url = images_url;
    }

    fn get_formatted_name(s: &String) -> String {
        let mut formatted_name = s.chars().nth(0).unwrap().to_uppercase().to_string();
        match s.get(1..) {
            None => (),
            Some(end_of_repo_name) => formatted_name.push_str(end_of_repo_name),
        };
        Project::replace_camel_case_by_spaces(&formatted_name.replace("-", " "))
    }

    fn replace_camel_case_by_spaces(s: &String) -> String {
        let mut formatted_s = String::new();
        let mut i: usize = 0;
        while i < s.len() {
            formatted_s.push(s.chars().nth(i).unwrap());
            if s.chars().nth(i).unwrap().is_ascii_lowercase()
                && i < s.len() - 1
                && s.chars().nth(i + 1).unwrap().is_ascii_uppercase()
            {
                formatted_s.push(' ');
                formatted_s.push(s.chars().nth(i + 1).unwrap());
                i += 1;
            }
            i += 1;
        }
        formatted_s
    }
}

impl From<GithubRepository> for Project {
    fn from(value: GithubRepository) -> Self {
        Project {
            name: value.name.clone(),
            full_name: value.full_name.clone(),
            formatted_name: Project::get_formatted_name(&value.name),
            url: value.html_url,
            teaser: value.description,
            description: None,
            cover_url: format!(
                "https://raw.githubusercontent.com/{}/master/cover.png",
                value.full_name
            ),
            topics: value.topics,
            stargazers_count: value.stargazers_count,
            watchers_count: value.watchers_count,
            images_url: Vec::new(),
        }
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} ⭐ {} 👀",
            self.formatted_name, self.stargazers_count, self.watchers_count
        )
    }
}
