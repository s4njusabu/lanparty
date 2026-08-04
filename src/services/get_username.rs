pub fn get_username() -> String {
    petname::petname(2, "-").unwrap_or("goofball".to_string())
}
