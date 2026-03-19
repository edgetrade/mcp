/// Prompt: User needs to create a password (input required)
pub fn create_password() -> String {
    "Create a password for your key (or press Enter to generate a random key): ".to_string()
}

/// Prompt: User needs to confirm password again
pub fn confirm_password() -> String {
    "Confirm password: ".to_string()
}

/// Prompt: User needs to confirm no-password key creation
pub fn confirm_no_password() {
    println!("Without a password...Continue? (Y/n)");
}
