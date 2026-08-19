pub fn default_username() -> String {
    loop {
        let name = petname::petname(2, "-").unwrap_or_else(|| "goofball".to_string());
        if name.len() <= 10 {
            return name;
        }
    }
}
