#!/bin/bash
# Notification Helper Functions
# Sends desktop/phone notifications using ntfy.sh
#
# Installation:
#   1. Set your ntfy topic: export NTFY_TOPIC="your-unique-topic-name"
#   2. Source this file in your shell config: source /path/to/notify-helpers.sh
#   3. Subscribe on your device:
#      - Phone: Install ntfy app, subscribe to your topic
#      - Desktop: Visit https://ntfy.sh/your-unique-topic-name
#
# Usage:
#   notify "Job completed!"
#   notify "Analysis done" "Title" high
#   long_command && notify "Command finished"
#   notifyme sleep 10  # Notify when command completes with timing

# Configuration
# Set your unique topic name - use something like your username + random string
# Example: export NTFY_TOPIC="lukeoc_hpc_$(whoami)"
NTFY_TOPIC="${NTFY_TOPIC:-}"
NTFY_SERVER="${NTFY_SERVER:-https://ntfy.sh}"

# Notification function
notify() {
    local message="$1"
    local title="${2:-O2 Notification}"
    local priority="${3:-default}"  # low, default, high, urgent
    local tags="${4:-computer,checkered_flag}"

    # Check if topic is set
    if [ -z "$NTFY_TOPIC" ]; then
        echo "ERROR: NTFY_TOPIC not set. Add to ~/.bashrc:"
        echo "  export NTFY_TOPIC=\"your-unique-topic-name\""
        return 1
    fi

    # Send notification via curl
    curl -s \
        -H "Title: $title" \
        -H "Priority: $priority" \
        -H "Tags: $tags" \
        -d "$message" \
        "$NTFY_SERVER/$NTFY_TOPIC" > /dev/null 2>&1

    local exit_code=$?

    if [ $exit_code -eq 0 ]; then
        echo "✓ Notification sent: $message"
    else
        echo "✗ Failed to send notification (curl exit code: $exit_code)"
    fi

    return $exit_code
}

# Notify when SLURM job completes (use in SLURM script)
notify_job_complete() {
    local job_name="${SLURM_JOB_NAME:-unknown}"
    local job_id="${SLURM_JOB_ID:-unknown}"
    local exit_code="${1:-0}"

    if [ "$exit_code" -eq 0 ]; then
        notify "Job '$job_name' (ID: $job_id) completed successfully" \
               "SLURM Job Complete" \
               "default" \
               "white_check_mark,computer"
    else
        notify "Job '$job_name' (ID: $job_id) failed with exit code $exit_code" \
               "SLURM Job Failed" \
               "high" \
               "x,warning"
    fi
}

# Notify with command timing
notify_with_time() {
    local start_time=$(date +%s)

    # Run the command
    "$@"
    local exit_code=$?

    local end_time=$(date +%s)
    local duration=$((end_time - start_time))

    # Format duration
    local duration_str
    if [ $duration -lt 60 ]; then
        duration_str="${duration}s"
    elif [ $duration -lt 3600 ]; then
        duration_str="$((duration / 60))m $((duration % 60))s"
    else
        duration_str="$((duration / 3600))h $(((duration % 3600) / 60))m"
    fi

    # Send notification
    if [ $exit_code -eq 0 ]; then
        notify "Command completed in $duration_str: $*" \
               "Command Complete" \
               "default"
    else
        notify "Command failed after $duration_str (exit: $exit_code): $*" \
               "Command Failed" \
               "high" \
               "x,warning"
    fi

    return $exit_code
}

# Test notification
test_notify() {
    echo "Testing notification system..."
    echo "Topic: $NTFY_TOPIC"
    echo "Server: $NTFY_SERVER"
    echo ""

    if [ -z "$NTFY_TOPIC" ]; then
        echo "ERROR: NTFY_TOPIC not set!"
        echo "Add this to your ~/.bashrc:"
        echo "  export NTFY_TOPIC=\"$(whoami)_o2_notifications\""
        echo ""
        echo "Then subscribe on your device:"
        echo "  Phone: Install ntfy app, subscribe to topic"
        echo "  Browser: Visit $NTFY_SERVER/$(whoami)_o2_notifications"
        return 1
    fi

    notify "Test notification from O2 - $(hostname)" \
           "Test Notification" \
           "default" \
           "bell,test_tube"

    echo ""
    echo "If you received a notification, setup is working!"
    echo "If not, check:"
    echo "  1. You're subscribed to topic: $NTFY_TOPIC"
    echo "  2. Visit: $NTFY_SERVER/$NTFY_TOPIC"
    echo "  3. curl is available: $(which curl)"
}

# Wrapper to notify when shell command completes
# Usage: notifyme sleep 10
notifyme() {
    notify_with_time "$@"
}

# Export functions so they're available in subshells
export -f notify
export -f notify_job_complete
export -f notify_with_time
export -f test_notify
export -f notifyme

# Mark as loaded (silent - no output on login)
export O2_NOTIFY_LOADED=1
