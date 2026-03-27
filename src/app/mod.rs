pub mod cli;
pub mod client;
pub mod handler;
pub mod runner;

use std::future::Future;
use std::pin::Pin;

use crate::client::IrisClient;
use crate::config::Config;
use crate::messages::success::CommandResult;

/// Key command function type.
pub type KeyCreateFn = fn(Config) -> CommandResult<()>;
pub type KeyUnlockFn = fn(Config) -> CommandResult<()>;
pub type KeyLockFn = fn(Config) -> CommandResult<()>;
pub type KeyUpdateFn =
    Box<dyn Fn(Config, &IrisClient) -> Pin<Box<dyn Future<Output = CommandResult<()>> + '_>> + Send + Sync>;
pub type KeyDeleteFn = fn(Config) -> CommandResult<()>;
