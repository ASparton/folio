pub struct Remote {
    name: String,
    projects_count: u16,
}

pub fn list_remotes(gh_auth_token: &String) -> Vec<Remote> {
    let mut remotes: Vec<Remote> = Vec::new();
    remotes
}