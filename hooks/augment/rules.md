---
type: always_apply
description: In Augment, use RTK by default: summary first, raw only when needed.
---

# RTK Usage

Default to `rtk <cmd>` for non-trivial shell output. Use raw commands only for tiny one-line checks or when exact raw formatting is intentionally required.

## High-priority commands

### Git

```bash
rtk git status
rtk git diff --stat
rtk git diff
rtk git log -n 20
```

### Search / files

```bash
rtk grep "pattern" .
rtk find "*.go" .
rtk ls src/
```

### Rust

```bash
rtk cargo test
rtk cargo clippy
rtk cargo build
rtk cargo check
```

### Go

```bash
rtk go test
rtk golangci-lint run
rtk go build
rtk go vet
```

### TypeScript / JavaScript

```bash
rtk vitest
rtk jest
rtk playwright test
rtk tsc
rtk lint
rtk next build
```

## Raw output / bypass

```bash
rtk proxy <cmd>             # Exact raw output, no filtering
rtk proxy sh -lc '...'      # Raw multi-command shell sequence
RTK_DISABLED=1 <cmd>        # One-off bypass without wrapping
```

Bare commands are acceptable only for tiny one-line checks such as `pwd`, `echo`, `command -v`, or `git branch --show-current`.

Do not double-wrap commands already starting with `rtk`.