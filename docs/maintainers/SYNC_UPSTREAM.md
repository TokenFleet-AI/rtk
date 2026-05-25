# Sync Upstream

This project is a fork of [rtk-ai/rtk](https://github.com/rtk-ai/rtk). The sync workflow keeps `master` and `develop` in sync with upstream changes.

## Automated sync

The workflow runs every Monday at 6:00 AM UTC automatically.

## Manual sync

1. Go to https://github.com/TokenFleet-AI/rtk/actions/workflows/sync-upstream.yml
2. Click **Run workflow**
3. Select branch `develop`
4. Click **Run workflow**

The workflow will:

1. Merge `rtk-ai/rtk/master` into your `master`
2. Merge `master` into `develop`
3. Push both branches

Conflicts are auto-resolved using `--ours` strategy (your fork's version wins). This preserves URL rebranding, workspace configuration, and other fork-specific changes.
