# rtk-registry

Command rewriting engine from the [RTK (Rust Token Killer)](https://github.com/TokenFleet-AI/rtk) project.

Rewrite shell commands to their RTK-optimized equivalents for token-efficient LLM filtering. Supports 200+ commands across git, cargo, npm/pnpm, go, python, docker, and more ecosystems.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
rtk-registry = "0.1"
```

## Usage

### Rewrite a Command

```rust
use rtk_registry::rewrite_command;

let rewritten = rewrite_command("git status", &[], &[]);
assert_eq!(rewritten, Some("rtk git status".to_string()));

// Unsupported command
let result = rewrite_command("htop", &[], &[]);
assert_eq!(result, None);

// Already has rtk prefix
let result = rewrite_command("rtk git log", &[], &[]);
assert_eq!(result, Some("rtk git log".to_string()));
```

### Compound Commands

```rust
use rtk_registry::rewrite_command;

// Chain commands are rewritten segment by segment
let result = rewrite_command("git status && cargo test", &[], &[]);
assert_eq!(result, Some("rtk git status && rtk cargo test".to_string()));

// Pipes work too
let result = rewrite_command("git diff | head -20", &[], &[]);
assert_eq!(result, Some("rtk git diff | head -20".to_string()));
```

### Check if RTK is Installed

```rust
use rtk_registry::{is_rtk_installed, RtkInstallStatus};

match is_rtk_installed() {
    RtkInstallStatus::Installed { path } => {
        println!("rtk is installed at: {}", path.display());
    }
    RtkInstallStatus::NotInstalled { install_hints } => {
        eprintln!("rtk is not installed.");
        eprintln!("Install with one of:");
        for cmd in &install_hints.install_commands {
            eprintln!("  {}", cmd);
        }
        eprintln!("Repository: {}", install_hints.repository);
        eprintln!("Docs: {}", install_hints.docs_url);
    }
}
```

### Require RTK (Fail if Not Installed)

```rust
use rtk_registry::require_rtk_installed;

let rtk_path = require_rtk_installed().map_err(|hints| {
    format!(
        "rtk not found. Install via: {}",
        hints.install_commands.join(" or ")
    )
})?;

println!("Using rtk at: {}", rtk_path.display());
```

### Exclude Commands and Transparent Prefixes

```rust
use rtk_registry::rewrite_command;

// Exclude specific commands from rewriting
let excluded = vec!["git push".to_string()];
let result = rewrite_command("git push origin main", &excluded, &[]);
assert_eq!(result, None);

// Transparent prefixes (wrapper commands)
let prefixes = vec!["docker exec mycontainer".to_string()];
let result = rewrite_command("docker exec mycontainer git status", &[], &prefixes);
assert_eq!(result, Some("docker exec mycontainer rtk git status".to_string()));
```

### Classify a Command

```rust
use rtk_registry::{classify_command, Classification};

match classify_command("git status") {
    Classification::Supported { rtk_equivalent, category, estimated_savings_pct, .. } => {
        println!("RTK equivalent: {}", rtk_equivalent);
        println!("Category: {}", category);
        println!("Estimated savings: {:.0}%", estimated_savings_pct);
    }
    Classification::Unsupported { base_command } => {
        println!("No RTK support for: {}", base_command);
    }
    Classification::Ignored => {
        println!("Command should be passed through unchanged");
    }
}
```

### Access the Rule Table

```rust
use rtk_registry::RULES;

println!("RTK supports {} command patterns", RULES.len());

for rule in RULES.iter().take(5) {
    println!("{} → {} ({}% savings)",
        rule.pattern,
        rule.rtk_cmd,
        rule.savings_pct
    );
}
```

## Public API

| Function | Description |
|----------|-------------|
| `rewrite_command(cmd, excluded, transparent_prefixes) -> Option<String>` | Rewrite a command to its RTK equivalent |
| `rewrite_command_with_config(cmd, excluded, transparent_prefixes, config) -> Option<String>` | Rewrite with custom warning callback |
| `classify_command(cmd) -> Classification` | Classify whether RTK handles this command |
| `is_rtk_installed() -> RtkInstallStatus` | Check if rtk binary is in PATH |
| `require_rtk_installed() -> Result<PathBuf, InstallHints>` | Require rtk, return install hints on failure |
| `has_heredoc(cmd) -> bool` | Detect heredoc in command (won't rewrite) |
| `split_command_chain(cmd) -> Vec<String>` | Split compound command on operators |
| `RULES: &[RtkRule]` | Static table of all rewrite rules (881+ patterns) |

## License

MIT
