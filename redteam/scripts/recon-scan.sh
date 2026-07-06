#!/usr/bin/env bash
# recon-scan.sh — scope-gated, rate-limited, READ-ONLY recon for the web-pentest skill.
# Refuses any host not present in engagement/scope.txt. Sends no payloads.
#
# Usage:  scripts/recon-scan.sh <host-or-url> [scope.txt]
#         scripts/recon-scan.sh --selftest
#
# Writes nothing; prints to stdout. Redirect into engagement/evidence/recon.md yourself.
set -euo pipefail

SCOPE_FILE="${2:-engagement/scope.txt}"

# host_in_scope <host> <scope-file> -> exit 0 if in scope.
# Matches exact hostname lines, and IPs falling inside CIDR lines. No wildcards.
host_in_scope() {
  local host="$1" scope="$2" line
  [ -f "$scope" ] || { echo "scope file not found: $scope" >&2; return 2; }
  host="$(printf '%s' "$host" | tr '[:upper:]' '[:lower:]')"
  while IFS= read -r line; do
    line="${line%%#*}"; line="$(printf '%s' "$line" | xargs || true)"   # strip comment + trim
    [ -z "$line" ] && continue
    if [ "$line" = "$host" ]; then return 0; fi
    # CIDR match for IP hosts, via python (portable, no ipcalc dependency)
    if printf '%s' "$line" | grep -q '/' && printf '%s' "$host" | grep -qE '^[0-9a-fA-F:.]+$'; then
      python3 - "$host" "$line" <<'PY' && return 0 || true
import ipaddress, sys
try:
    if ipaddress.ip_address(sys.argv[1]) in ipaddress.ip_network(sys.argv[2], strict=False):
        sys.exit(0)
except ValueError:
    pass
sys.exit(1)
PY
    fi
  done < "$scope"
  return 1
}

# Extract a bare hostname from a URL or pass through a raw host.
extract_host() {
  local t="$1"
  case "$t" in
    *://*) printf '%s' "$t" | sed -E 's#^[a-zA-Z]+://([^/:]+).*#\1#' ;;
    *)     printf '%s' "$t" | sed -E 's#[:/].*##' ;;
  esac
}

selftest() {
  local tmp; tmp="$(mktemp)"
  printf 'staging.example.com\n10.0.5.0/24\n# comment\n127.0.0.1\n' > "$tmp"
  local pass=0 fail=0
  check() { if host_in_scope "$1" "$tmp"; then r=in; else r=out; fi
            if [ "$r" = "$2" ]; then pass=$((pass+1)); else fail=$((fail+1)); echo "FAIL: $1 -> $r (want $2)"; fi; }
  check staging.example.com in
  check STAGING.EXAMPLE.COM in      # case-insensitive
  check prod.example.com    out     # not listed, no wildcard
  check 10.0.5.42           in      # inside CIDR
  check 10.0.6.42           out     # outside CIDR
  check 127.0.0.1           in
  check "$(extract_host https://staging.example.com:8443/login)" in
  rm -f "$tmp"
  echo "selftest: $pass passed, $fail failed"
  [ "$fail" -eq 0 ]
}

[ "${1:-}" = "--selftest" ] && { selftest; exit $?; }
[ $# -ge 1 ] || { echo "usage: $0 <host-or-url> [scope.txt] | --selftest" >&2; exit 64; }

TARGET="$1"
HOST="$(extract_host "$TARGET")"

if ! host_in_scope "$HOST" "$SCOPE_FILE"; then
  echo "REFUSED: '$HOST' is not in $SCOPE_FILE — add it (and confirm authorization) first." >&2
  exit 3
fi

echo "# Recon: $HOST  ($(date -u +%FT%TZ))"
echo "## HTTP headers"
curl -sS -I --max-time 15 "$TARGET" 2>&1 || echo "(headers failed)"

echo; echo "## Tech fingerprint (whatweb)"
if command -v whatweb >/dev/null; then whatweb --max-threads 1 -a 1 "$TARGET" 2>&1 || echo "(whatweb failed)"
else echo "(whatweb not installed — skip)"; fi

echo; echo "## Open ports (nmap, rate-limited, top 100)"
if command -v nmap >/dev/null; then
  # --max-rate keeps it gentle; -T2 polite timing; no scripts (read-only surface only)
  nmap -Pn -T2 --max-rate 50 --top-ports 100 "$HOST" 2>&1 || echo "(nmap failed)"
else echo "(nmap not installed — skip)"; fi

echo; echo "## robots.txt"
curl -sS --max-time 10 "${TARGET%/}/robots.txt" 2>&1 | head -50 || true
