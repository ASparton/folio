use std::fs;

use super::super::get_readme_content;

const TEMP_TEST_DIR_PATH: &str = "./temp_test_dir";

#[test]
fn test_get_readme_content_existing_file() {
    // Prepare
    fs::create_dir(TEMP_TEST_DIR_PATH).unwrap();
    fs::write(
        format!("{}/{}", TEMP_TEST_DIR_PATH, "README.md"),
        "This is a test README file",
    )
    .unwrap();

    // Act
    let result = get_readme_content(TEMP_TEST_DIR_PATH);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "This is a test README file");

    // Prepare 2
    fs::remove_file(format!("{}/{}", TEMP_TEST_DIR_PATH, "README.md")).unwrap();
    fs::write(
        format!("{}/{}", TEMP_TEST_DIR_PATH, "Readme.md"),
        "This is a test Readme file",
    )
    .unwrap();

    // Act 2
    let result = get_readme_content(TEMP_TEST_DIR_PATH);

    // Assert 2
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "This is a test Readme file");

    // Prepare 3
    fs::remove_file(format!("{}/{}", TEMP_TEST_DIR_PATH, "Readme.md")).unwrap();
    fs::write(
        format!("{}/{}", TEMP_TEST_DIR_PATH, "readme.md"),
        "This is a test readme file",
    )
    .unwrap();

    // Act 3
    let result = get_readme_content(TEMP_TEST_DIR_PATH);

    // Assert 3
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "This is a test readme file");

    // Clean
    fs::remove_file(format!("{}/{}", TEMP_TEST_DIR_PATH, "readme.md")).unwrap();
    fs::remove_dir(TEMP_TEST_DIR_PATH).unwrap();
}

#[test]
fn test_get_readme_content_non_existing_file() {
    // Act
    let result = get_readme_content(TEMP_TEST_DIR_PATH);

    // Assert
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "No README file found");
}
