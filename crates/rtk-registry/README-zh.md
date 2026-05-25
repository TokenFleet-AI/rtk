# rtk-registry

[RTK (Rust Token Killer)](https://github.com/TokenFleet-AI/rtk) 项目的命令重写引擎库。

将 shell 命令重写为 RTK 优化格式，实现 token 高效的 LLM 过滤。支持 200+ 命令，涵盖 git、cargo、npm/pnpm、go、python、docker 等生态。

## 快速开始

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
rtk-registry = "0.1"
```

## 使用示例

### 重写命令

```rust
use rtk_registry::rewrite_command;

let rewritten = rewrite_command("git status", &[], &[]);
assert_eq!(rewritten, Some("rtk git status".to_string()));

// 不支持的命令返回 None
let result = rewrite_command("htop", &[], &[]);
assert_eq!(result, None);

// 已有 rtk 前缀的命令原样返回
let result = rewrite_command("rtk git log", &[], &[]);
assert_eq!(result, Some("rtk git log".to_string()));
```

### 复合命令

```rust
use rtk_registry::rewrite_command;

// 链式命令逐段重写
let result = rewrite_command("git status && cargo test", &[], &[]);
assert_eq!(result, Some("rtk git status && rtk cargo test".to_string()));

// 管道命令也可以
let result = rewrite_command("git diff | head -20", &[], &[]);
assert_eq!(result, Some("rtk git diff | head -20".to_string()));
```

### 检查 RTK 是否已安装

```rust
use rtk_registry::{is_rtk_installed, RtkInstallStatus};

match is_rtk_installed() {
    RtkInstallStatus::Installed { path } => {
        println!("rtk 已安装在: {}", path.display());
    }
    RtkInstallStatus::NotInstalled { install_hints } => {
        eprintln!("rtk 未安装，请执行以下命令之一:");
        for cmd in &install_hints.install_commands {
            eprintln!("  {}", cmd);
        }
        eprintln!("仓库地址: {}", install_hints.repository);
        eprintln!("文档: {}", install_hints.docs_url);
    }
}
```

### 要求 RTK 必须安装（未安装时报错）

```rust
use rtk_registry::require_rtk_installed;

let rtk_path = require_rtk_installed().map_err(|hints| {
    format!(
        "rtk 未安装。请通过以下方式安装: {}",
        hints.install_commands.join(" 或 ")
    )
})?;

println!("rtk 路径: {}", rtk_path.display());
```

### 排除命令和透明前缀

```rust
use rtk_registry::rewrite_command;

// 排除特定命令不被重写
let excluded = vec!["git push".to_string()];
let result = rewrite_command("git push origin main", &excluded, &[]);
assert_eq!(result, None);

// 透明前缀（包装命令），重写会保留前缀
let prefixes = vec!["docker exec mycontainer".to_string()];
let result = rewrite_command("docker exec mycontainer git status", &[], &prefixes);
assert_eq!(result, Some("docker exec mycontainer rtk git status".to_string()));
```

### 命令分类

```rust
use rtk_registry::{classify_command, Classification};

match classify_command("git status") {
    Classification::Supported { rtk_equivalent, category, estimated_savings_pct, .. } => {
        println!("RTK 等价命令: {}", rtk_equivalent);
        println!("分类: {}", category);
        println!("预估节省: {:.0}%", estimated_savings_pct);
    }
    Classification::Unsupported { base_command } => {
        println!("无 RTK 支持: {}", base_command);
    }
    Classification::Ignored => {
        println!("命令应原样传递");
    }
}
```

### 访问规则表

```rust
use rtk_registry::RULES;

println!("RTK 支持 {} 个命令模式", RULES.len());

for rule in RULES.iter().take(5) {
    println!("{} → {} (节省 {:.0}%)",
        rule.pattern,
        rule.rtk_cmd,
        rule.savings_pct
    );
}
```

## 公共 API

| 函数 | 描述 |
|------|------|
| `rewrite_command(cmd, excluded, transparent_prefixes) -> Option<String>` | 重写命令为 RTK 等价形式 |
| `rewrite_command_with_config(cmd, excluded, transparent_prefixes, config) -> Option<String>` | 带自定义回调的重写 |
| `classify_command(cmd) -> Classification` | 分类命令是否有 RTK 支持 |
| `is_rtk_installed() -> RtkInstallStatus` | 检查 rtk 二进制是否在 PATH 中 |
| `require_rtk_installed() -> Result<PathBuf, InstallHints>` | 要求已安装，未安装时返回安装提示 |
| `has_heredoc(cmd) -> bool` | 检测命令中的 heredoc（不会重写） |
| `split_command_chain(cmd) -> Vec<String>` | 按操作符分割复合命令 |
| `RULES: &[RtkRule]` | 静态重写规则表（881+ 模式） |

## 支持的生态系统

| 生态 | 命令 | 预估节省 |
|------|------|----------|
| Git | git, gh, gt, diff | 80%+ |
| Rust | cargo, cargo test | 90%+ |
| JS/TS | npm, pnpm, vitest, jest, next, playwright, prisma | 70%+ |
| Python | ruff, pytest, mypy, pip | 75%+ |
| Go | go, golangci-lint | 65%+ |
| 容器 | docker, kubectl | 60%+ |
| 系统 | ls, tree, find, grep, env | 70%+ |
| Ruby | rake, rspec, rubocop | 80%+ |
| .NET | dotnet, binlog, trx | 60%+ |

## 许可证

MIT
