#!/bin/sh
# Legacy shim for machines whose ~/.claude/settings.json still references
# ~/.claude/hooks/. The real hook now lives in the lab-config plugin; running
# it triggers `config-agent-tool update --agent claude`, which migrates this
# machine to the plugin-based layout.
exec "$(cd "$(dirname "$0")" && pwd -P)/../plugin/lab-config/hooks/update-config.sh" "$@"
