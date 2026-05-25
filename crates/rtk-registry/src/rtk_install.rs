//! Check whether the `rtk` binary is available on PATH.
//!
//! Provides installation hints when the binary is not found, so library
//! consumers can surface actionable errors to their users.

use std::path::PathBuf;

/// Installation status of the `rtk` binary.
#[derive(Debug, Clone)]
pub enum RtkInstallStatus {
    /// `rtk` is on PATH.
    Installed { path: PathBuf },
    /// `rtk` is not on PATH.  Installation hints are provided.
    NotInstalled { install_hints: InstallHints },
}

/// Actionable hints for installing `rtk`.
#[derive(Debug, Clone)]
pub struct InstallHints {
    /// Recommended install commands.
    pub install_commands: Vec<&'static str>,
    /// GitHub repository.
    pub repository: &'static str,
    /// Documentation URL.
    pub docs_url: &'static str,
}

/// Check if the `rtk` binary is on PATH.
pub fn is_rtk_installed() -> RtkInstallStatus {
    match which::which("rtk") {
        Ok(path) => RtkInstallStatus::Installed { path },
        Err(_) => RtkInstallStatus::NotInstalled {
            install_hints: InstallHints {
                install_commands: vec!["cargo install rtk", "brew install TokenFleet-AI/rtk/rtk"],
                repository: "https://github.com/TokenFleet-AI/rtk",
                docs_url: "https://www.rtk-ai.app",
            },
        },
    }
}

/// Require the `rtk` binary to be on PATH.
///
/// Returns the binary path when installed, or [`InstallHints`] when not.
pub fn require_rtk_installed() -> Result<PathBuf, InstallHints> {
    match is_rtk_installed() {
        RtkInstallStatus::Installed { path } => Ok(path),
        RtkInstallStatus::NotInstalled { install_hints } => Err(install_hints),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rtk_installed_returns_enum() {
        let status = is_rtk_installed();
        match status {
            RtkInstallStatus::Installed { path } => {
                assert!(path.ends_with("rtk"));
            }
            RtkInstallStatus::NotInstalled { install_hints } => {
                assert!(!install_hints.install_commands.is_empty());
                assert!(install_hints.repository.starts_with("http"));
                assert!(install_hints.docs_url.starts_with("http"));
            }
        }
    }

    #[test]
    fn test_require_rtk_installed_result_type() {
        let result = require_rtk_installed();
        match result {
            Ok(path) => {
                assert!(path.ends_with("rtk"));
            }
            Err(hints) => {
                assert!(!hints.install_commands.is_empty());
            }
        }
    }
}
