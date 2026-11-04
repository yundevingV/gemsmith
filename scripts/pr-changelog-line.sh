#!/usr/bin/env bash
set -euo pipefail

PR_TITLE="${PR_TITLE:?}"
PR_NUMBER="${PR_NUMBER:?}"
PR_URL="${PR_URL:?}"
PR_BODY="${PR_BODY:-}"

if ! printf '%s' "$PR_BODY" | grep -q '^## Changelog'; then
  echo "::error::PR 본문에 ## Changelog 섹션이 없습니다."
  exit 1
fi

NOTE=$(
  printf '%s' "$PR_BODY" |
    sed -n '/^## Changelog/,/^## /p' |
    tail -n +2 |
    sed '/^## /d' |
    sed '/^[[:space:]]*$/d' |
    sed '/^<!--/d' |
    head -n 1
)

if [ -z "$NOTE" ]; then
  NOTE="$PR_TITLE"
  echo "::notice::Changelog 칸이 비어 있어 PR 제목을 사용합니다." >&2
fi

NOTE="${NOTE//$'\r'/}"
echo "- ${NOTE} ([#${PR_NUMBER}](${PR_URL}))"
