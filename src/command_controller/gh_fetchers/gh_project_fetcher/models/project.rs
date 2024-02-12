use serde::Serialize;
use tabled::Tabled;

use crate::command_controller::gh_fetchers::gh_project_fetcher::models::github_repository::GithubRepository;
use crate::command_displayer::{display_firsts_string_vec, display_count, display_written_length};

/// Describes a portfolio project.
#[derive(Debug, Serialize, Tabled)]
pub struct Project {
    #[tabled(skip)]
    name: String,

    #[tabled(skip)]
    full_name: String,

    #[tabled(rename = "Name")]
    formatted_name: String,

    #[tabled(skip)]
    url: String,

    #[tabled(rename = "Teaser ✍️", display_with = "display_written_length")]
    teaser: Option<String>,

    #[tabled(rename = "Description 📄", display_with = "display_written_length")]
    description: Option<String>,

    #[tabled(skip)]
    cover_url: String,

    #[tabled(rename = "Topics 🪧", display_with = "display_firsts_string_vec")]
    topics: Vec<String>,

    #[tabled(rename = "Illustrations 🖼️", display_with = "display_count")]
    images_url: Vec<String>,

    #[tabled(rename = "Stargazers ⭐")]
    stargazers_count: u32,

    #[tabled(rename = "Watcher 👀")]
    watchers_count: u32,
}

impl Project {
    pub fn get_name(&self) -> String {
        self.name.clone()
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
