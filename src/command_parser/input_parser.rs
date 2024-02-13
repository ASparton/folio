pub fn is_json_display(input_args: &Vec<String>) -> bool {
    input_args.contains(&String::from("--json"))
}

pub fn first_arg_is_valid_path(input_args: &Vec<String>) -> bool {
    input_args.contains(&String::from("--json"))
}
