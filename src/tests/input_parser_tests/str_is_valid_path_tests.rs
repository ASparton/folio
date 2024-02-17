use crate::command_parser::input_parser::str_is_valid_path_to_folder;

#[test]
fn test_str_is_valid_path_to_folder_returns_false_when_empty() {
    assert_eq!(str_is_valid_path_to_folder(&"".to_string()), false);
}

#[test]
fn test_str_is_valid_path_to_folder_returns_false_when_invalid_path() {
    assert_eq!(
        str_is_valid_path_to_folder(&"this is n0t A path at all".to_string()),
        false
    );
}

#[test]
fn test_str_is_valid_path_to_folder_returns_false_when_nonexistent_folder() {
    assert_eq!(
        str_is_valid_path_to_folder(&"/path/to/nonexistent/folder".to_string()),
        false
    );
}

#[test]
fn test_str_is_valid_path_to_folder_valid_folder() {
    assert_eq!(
        str_is_valid_path_to_folder(&"/".to_string()),
        true
    );
    assert_eq!(
        str_is_valid_path_to_folder(&"./src/".to_string()),
        true
    );
    assert_eq!(
        str_is_valid_path_to_folder(&"./src/error".to_string()),
        true
    );
    assert_eq!(
        str_is_valid_path_to_folder(&".".to_string()),
        true
    );
    assert_eq!(
        str_is_valid_path_to_folder(&"..".to_string()),
        true
    );
}
