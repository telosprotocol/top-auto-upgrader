// Interacting with system, operate files or topio binary.
// Execute commands.

mod file;
mod topio;

/// standard file io methods. Used for `config.json`.
pub(crate) use file::{read_file, write_file};
pub(crate) use topio::{JoinStatus, ProcessStatus, TopioCommands};
