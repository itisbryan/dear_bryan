# Scope Enforcement (redteam reference)

This skill is dangerous because it drives network tools, sometimes across several turns. The single most important rule: **every active request must target a host the operator authorized.** This file is the procedure.

## The three authorities

1. `engagement/authorization.md` — what the operator wrote down (prose).
2. `engagement/scope.txt` — the machine-readable allowlist.
3. The implicit context — "I'm running inside the operator's environment."

If any of these disagree, **STOP and ask.** Do not try to reconcile them yourself.

## scope.txt format

One target per line. `#` for comments. **No wildcards.**

```
# Hostnames — resolved at use time
localhost
127.0.0.1
::1
staging.example.com
api-staging.example.com

# CIDR — internal labs only, requires operator OK in writing
192.168.50.0/24
10.0.5.0/24
```

Wildcards are deliberately unsupported. If you need `*.staging.example.com`, list each host explicitly — subdomain wildcards in scope are how unauthorized testing happens.

## Host-extraction rules

Before any active request, extract the target host from the command/URL and confirm it's in scope.

| Surface | Where the host lives | Note |
|---|---|---|
| `curl URL` | the URL | `urlparse(url).hostname.lower()` |
| `curl --resolve HOST:PORT:ADDR` | — | **reject** — `--resolve` overrides scope/DNS |
| `nmap TARGET…` | each TARGET arg | check every one |
| `whatweb URL` | the URL | |
| browser navigation | the URL | re-check on every redirect |
| tool-driven HTTP (sqlmap, ffuf, gobuster) | `-u` / `-h` / target arg | tool-dependent |

For raw IPs, check against CIDR entries: `ipaddress.ip_address(host) in ipaddress.ip_network(cidr)`.

## Pre-send checklist (every active request)

1. Did you extract the host correctly? (URL host — not the `Host:` header, not a `--resolve` alias.)
2. Is that host an exact match in `scope.txt`, or does its resolved IP fall in a scope CIDR?
3. If following a redirect, did you re-check scope on the **redirect** URL?
4. If this is the second hop of an SSRF probe, is the **inner** URL in scope? (Usually not — that's the point. Don't auto-fire.)
5. Did the operator approve this payload class? (Read-only recon is auto-OK; destructive payloads need explicit OK.)

Any "no" or "not sure" → STOP and ask.

## Looks in-scope but isn't

- **Redirects to a parent/sister host.** `staging.example.com` → `auth.example.com` is a *different* host. Re-confirm.
- **CNAMEs.** `app.staging.example.com` may CNAME to `prod-cluster.aws.example.com`. Resolve and check the IP, not just the name.
- **Cloud metadata IPs.** `169.254.169.254` and friends belong in no sane scope.txt. An SSRF candidate resolving there means you're hitting a real cloud host — needs explicit approval (see guardrail 4).
- **`localhost`/`127.0.0.1` on a shared box.** Loopback may be a *different* service than you think (the agent's own host, not the target). Confirm what's actually listening.
