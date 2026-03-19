use crate::utils::urls::DOCS_BASE_URL;

/// Prints connection failed message (for single-argument error)
pub fn connection_failed(err: &str) {
    eprintln!("Failed to connect to Edge servers: {}", err);
}

/// Prints connection failed message (for two-argument verbose logging)
pub fn connection_failed_url_key(url: &str, _key: &str) {
    eprintln!("[edge] connecting to {}", url);
}

/// Prints query error message
pub fn query_error(path: &str, err: &str) {
    eprintln!("[edge] ✗ {} (query/mutation): {}", path, err);
}

/// Prints mutation error message
pub fn mutation_error(path: &str, input: &str, err: &str) {
    eprintln!("Mutation error at path '{}' with input '{}': {}", path, input, err);
}

/// Prints subscription error message
pub fn subscription_error(path: &str, id: &str, err: &str) {
    eprintln!("Subscription error at path '{}' with id '{}': {}", path, id, err);
}

/// Prints reconnect failed message
pub fn reconnect_failed(path: &str, id: &str) {
    eprintln!("Reconnect failed at path '{}' with id '{}'", path, id);
}

/// Prints auth error message
pub fn auth_error(path: &str, id: &str, err: &str) {
    eprintln!("Auth error at path '{}' with id '{}': {}", path, id, err);
}

/// Prints SSE parse error message
pub fn sse_parse_error(data: &str) {
    eprintln!("SSE parse error: '{}'", data);
}

/// Prints manifest save error message
pub fn manifest_save_error(err: &str) {
    eprintln!("Failed to save manifest: {}", err);
}

/// Prints manifest parse error message
pub fn manifest_parse_error(err: &str) {
    eprintln!("Manifest parse error: {}", err);
}

/// Prints MCP server error message
pub fn mcp_server_error(err: &str) {
    eprintln!("MCP server error: {}", err);
}

/// Prints deprecated SSE transport warning
pub fn deprecated_transport_sse() {
    eprintln!("[edge] --transport sse is deprecated...");
}

/// Prints key command error message
pub fn key_command_error(command: &str, chain: &str) {
    eprintln!("[edge] key {} failed: {}", command, chain);
}

/// Prints invalid chain type error message
pub fn invalid_chain_type() {
    eprintln!("[edge] error: chain_type must be 'evm' or 'svm'");
}

/// Prints wallet command error message
pub fn wallet_command_error(command: &str, err: &str) {
    eprintln!("[edge] wallet {} failed: {}", command, err);
}

/// Prints create directory error message
pub fn create_dir_error(err: &str) {
    eprintln!("[edge] failed to create directory: {}", err);
}

/// Prints write skill error message
pub fn write_skill_error(err: &str) {
    eprintln!("[edge] failed to write skill: {}", err);
}

/// Prints skill installed message
pub fn skill_installed(name: &str, dir: &str) {
    eprintln!("[edge] installed skill '{}' to {}", name, dir);
}

/// Prints skill not found error message
pub fn skill_not_found(name: &str) {
    eprintln!("[edge] skill '{}' not found in manifest", name);
}

/// Prints pinging URL message
pub fn pinging(ping_url: &str) {
    eprintln!("[edge] pinging {}", ping_url);
}

/// Prints ping failed status message
pub fn ping_failed_status(status: &str) {
    eprintln!("Ping failed with status: {}", status);
}

/// Prints ping failed error message
pub fn ping_failed_error(err: &str) {
    eprintln!("Ping failed: {}", err);
}

/// Prints API key required error message
pub fn api_key_required() {
    eprintln!("Error: API key required. Set EDGE_API_KEY or use --api-key");
}

/// Prints API key documentation URL
pub fn api_key_docs_url() {
    println!("See: {}/authentication", DOCS_BASE_URL);
}

/// Prints HTTP server starting message
pub fn http_server_starting(addr: &str, path: &str) {
    println!("Starting HTTP server on http://{}{}", addr, path);
}

/// Prints alert registered message
pub fn alert_registered(name: &str, id: u64, summary: &str) {
    eprintln!("[edge] ✓ alert '{}' (id={}) → {}", name, id, summary);
}

/// Prints fetch HTTP error message
pub fn fetch_http_error(status: u16) {
    eprintln!("[edge] manifest fetch failed: HTTP {}", status);
}

/// Prints session error message
pub fn session_error(err: &str) {
    eprintln!("Session error: {}", err);
}

/// Prints fetch error message
pub fn fetch_error(url: &str) {
    eprintln!("[edge] manifest fetch error: {}", url);
}

/// Prints API key invalid message
pub fn api_key_invalid() {
    eprintln!("Error: Invalid API key. See: {}/authentication", DOCS_BASE_URL);
}

/// Prints Iris connection failed message
pub fn iris_connection_failed(err: &str) {
    eprintln!("Failed to connect to Iris: {}", err);
}

/// Prints manifest load error message
pub fn manifest_load_error(err: &str) {
    eprintln!("Failed to load manifest: {}", err);
}
