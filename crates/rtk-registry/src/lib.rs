//! `rtk-registry` — Command rewriting engine from the RTK (Rust Token Killer) project.
//!
//! This library exposes the command rewriting logic used by the `rtk` CLI,
//! allowing third-party tools to rewrite commands programmatically without
//! shelling out to the `rtk` binary.
//!
//! # Quick start
//!
//! ```rust
//! use rtk_registry::rewrite_command;
//!
//! let rewritten = rewrite_command("git status", &[], &[]);
//! assert_eq!(rewritten, Some("rtk git status".to_string()));
//! ```
//!
//! # Check if RTK is installed
//!
//! ```rust
//! use rtk_registry::{is_rtk_installed, RtkInstallStatus};
//!
//! match is_rtk_installed() {
//!     RtkInstallStatus::Installed { path } => {
//!         println!("rtk is installed at: {}", path.display());
//!     }
//!     RtkInstallStatus::NotInstalled { install_hints } => {
//!         eprintln!("rtk is not installed.");
//!         eprintln!("Install with one of:");
//!         for cmd in &install_hints.install_commands {
//!             eprintln!("  {}", cmd);
//!         }
//!     }
//! }
//! ```

mod lexer;
mod registry;
mod rtk_install;
mod rules;
pub mod status;

// Public API from registry.rs
pub use registry::{
    category_avg_tokens, classify_command, cmd_has_rtk_disabled_prefix, has_heredoc,
    prefix_contains_rtk_disabled, rewrite_command, rewrite_command_with_config,
    split_command_chain, strip_disabled_prefix, Classification, RewriteConfig,
};

// Public API from rules.rs
pub use rules::{RtkRule, IGNORED_EXACT, IGNORED_PREFIXES, RULES};

// Public API from status.rs
pub use status::RtkStatus;

// Public API from rtk_install.rs
pub use rtk_install::{is_rtk_installed, require_rtk_installed, InstallHints, RtkInstallStatus};
