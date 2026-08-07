#!/bin/sh
# Legacy shim for machines whose ~/.claude/settings.json still references
# ~/.claude/hooks/. The real hook now lives in the lab-config plugin.
exec "$(cd "$(dirname "$0")" && pwd -P)/../plugin/lab-config/hooks/notify.sh" "$@"
