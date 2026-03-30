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

/// Prompt the user for input.
pub fn prompt_user(message: &str) -> super::success::CommandResult<String> {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| super::error::CommandError::Io(e.to_string()))?;
    Ok(input.trim().to_string())
}

/// Prompt the user for a number.
pub fn prompt_number(message: &str) -> super::success::CommandResult<u64> {
    let input = prompt_user(message)?;
    input
        .parse::<u64>()
        .map_err(|_| super::error::CommandError::InvalidInput("Please enter a valid number".to_string()))
}

pub fn prompt_secret_number(message: &str) -> super::success::CommandResult<u64> {
    let input = rpassword::prompt_password(message)?;
    input
        .parse::<u64>()
        .map_err(|_| super::error::CommandError::InvalidInput("Please enter a valid number".to_string()))
}
