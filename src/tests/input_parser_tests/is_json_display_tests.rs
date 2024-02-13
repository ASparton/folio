use crate::command_parser::input_parser::is_json_display;

#[test]
fn test_is_json_display_returns_true_when_input_args_contains_json_flag() {
    let input_args = vec![
        String::from("folio"),
        String::from("remote"),
        String::from("list"),
        String::from("--json"),
    ];
    assert!(is_json_display(&input_args));
}

#[test]
fn test_is_json_display_returns_false_when_input_args_contains_json_flag() {
    let mut input_args = vec![
        String::from("folio"),
        String::from("remote"),
        String::from("list"),
    ];
    assert_eq!(is_json_display(&input_args), false);

    input_args = vec![
        String::from("folio"),
        String::from("remote"),
        String::from("list"),
        String::from("--jsons"),
    ];
    assert_eq!(is_json_display(&input_args), false);

    input_args = vec![
        String::from("folio"),
        String::from("remote"),
        String::from("list"),
        String::from("--jsons"),
    ];
    assert_eq!(is_json_display(&input_args), false);
}
