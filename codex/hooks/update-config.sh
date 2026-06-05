#!/bin/sh
# Silently refresh managed Codex config at startup without owning config.toml.

codex_home=${CODEX_HOME:-"$HOME/.codex"}
tool=$codex_home/bin/config-agent-tool

if [ ! -e "$tool" ]; then
    exit 0
fi

set -m 2>/dev/null || :

kill_tree() {
    signal=$1
    parent=$2
    for child in $(ps -e -o pid= -o ppid= 2>/dev/null | awk -v parent="$parent" '$2 == parent { print $1 }'); do
        kill_tree "$signal" "$child"
    done
    kill -s "$signal" "$parent" 2>/dev/null
}

kill_update() {
    signal=$1
    kill_tree "$signal" "$update_pid"
    kill -s "$signal" "-$update_pid" 2>/dev/null
}

"$tool" update --agent codex >/dev/null 2>&1 &
update_pid=$!

(
    sleep 5
    kill_update TERM
    sleep 1
    kill_update KILL
) >/dev/null 2>&1 &
watchdog_pid=$!

wait "$update_pid" 2>/dev/null
kill -TERM "$watchdog_pid" 2>/dev/null
wait "$watchdog_pid" 2>/dev/null

exit 0
