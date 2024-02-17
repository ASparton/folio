use std::fs;

pub fn is_json_display(input_args: &Vec<String>) -> bool {
    input_args.contains(&String::from("--json"))
}

pub fn str_is_valid_path_to_folder(to_validate: &String) -> bool {
    match fs::metadata(to_validate) {
        Ok(_) => true,
        Err(_) => false,
    }
}
