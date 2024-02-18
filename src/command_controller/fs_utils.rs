use std::fs;

pub fn get_readme_content(dir_path: &str) -> Result<String, std::io::Error> {
    let readme_paths = vec![
        format!("{}/README.md", dir_path),
        format!("{}/Readme.md", dir_path),
        format!("{}/readme.md", dir_path),
    ];

    for readme_path in readme_paths {
        let readme_content = fs::read_to_string(readme_path);
        if readme_content.is_ok() {
            return Ok(readme_content.unwrap());
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No README file found",
    ))
}

#[cfg(test)]
#[path = "./fs_utils_tests/mod.rs"]
mod fs_utils_tests;
