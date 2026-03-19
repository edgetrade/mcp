//! Desktop key management commands (keyring-based).

pub mod create;
pub mod delete;
pub mod lock;
pub mod unlock;
pub mod update;

pub use create::keyring_create;
pub use delete::keyring_delete;
pub use lock::keyring_lock;
pub use unlock::keyring_unlock;
pub use update::keyring_update;
