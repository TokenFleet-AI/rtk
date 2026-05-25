//! Data types for reporting which commands RTK can and cannot optimize.

use serde::Serialize;

/// RTK support status for a command.
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum RtkStatus {
    /// Dedicated handler with filtering (e.g., git status → git.rs:run_status())
    Existing,
    /// Works via external_subcommand passthrough, no filtering (e.g., cargo fmt → Other)
    Passthrough,
    /// RTK doesn't handle this command at all
    NotSupported,
}

impl RtkStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RtkStatus::Existing => "existing",
            RtkStatus::Passthrough => "passthrough",
            RtkStatus::NotSupported => "not-supported",
        }
    }
}
