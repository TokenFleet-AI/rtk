# Augment Integration

This integration uses Augment's native rules directories (`.augment/rules/` or `~/.augment/rules/`).
It is rules-based, not transparent command rewrite.
Current Augment hooks can intercept tools, but cannot yet mutate tool input.
RTK therefore guides Augment to prefer `rtk <cmd>` instead of rewriting commands automatically.
Future work may add strict hooks or transparent rewrite when upstream supports input mutation.