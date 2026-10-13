#!/bin/bash
set -euo pipefail
count=0
if [[ -f "$TEST_ROOT/count" ]]; then read -r count < "$TEST_ROOT/count"; fi
count=$((count + 1))
printf '%s\n' "$count" > "$TEST_ROOT/count"
printf '%s\0' "$@" > "$TEST_ROOT/call-$count"
if [[ "${2-}" == process-info ]]; then
  printf '%s' "${TEST_REPLY-}"
  printf '%s' 'probe diagnostic' >&2
  exit "${TEST_PROBE_STATUS-0}"
fi
printf '%s' 'action output'
printf '%s' 'action diagnostic' >&2
exit "${TEST_ACTION_STATUS-0}"
