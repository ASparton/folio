use std::env;

pub fn get_input_args() -> Vec<String> {
  let mut cmd_input: Vec<String> = env::args().collect();
	if cmd_input.len() > 0 {
		cmd_input.remove(0);
	}
  cmd_input
}