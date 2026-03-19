pub fn keyring_unavailable() {
    eprintln!("⚠️  WARNING: Keyring unavailable, falling back to file storage");
}

pub fn file_storage_fallback() {
    eprintln!(
        "⚠️  WARNING: Using file-based storage. To use keyring, remove or set session.use_keyring = true in config"
    );
}
