---
name: redteam
description: Authorized web-application penetration testing — recon, vulnerability analysis, proof-based exploitation, and a professional report. "No exploit, no report": every finding needs reproducible evidence. Hard guardrails for authorization, scope, and secret-leakage. Use when asked to "pentest [URL]", "security-test this web app", "find vulns in [URL]", or "OWASP-test" a running application the user owns or is authorized to test.
---

# Web Application Penetration Testing

A phased workflow for active testing of a running web app. Built on three rules:

1. **No exploit, no report** — every reported finding carries reproducible evidence (request, response, reproducer). Pattern-matches and hunches are not findings.
2. **Bounded scope** — every active request goes against a host the operator pre-declared in `engagement/scope.txt`. Off-scope hosts are refused, not reconciled.
3. **Bypass exhaustion before dismissal** — a blocked payload is not a clean bill of health until you've tried the bypass set for that class.

This produces **reference-quality attack evidence and a report**. It is for authorized testing only.

---

## ⚠️ Hard Guardrails — read before every engagement

Violating any of these can be illegal and invalidates the engagement.

1. **Authorization gate.** Before the first active request in a session, confirm with the user, in writing, that they own or have written authorization to test the target. Record it in `engagement/authorization.md` (template in `templates/authorization.md`). No acknowledgement → no active scanning. Reading a public page with `curl` is fine; sending a payload is not.

2. **Scope allowlist.** Maintain `engagement/scope.txt` — one hostname or CIDR per line, no wildcards. Every `nmap`, `curl`, `whatweb`, browser navigation, or payload-bearing request must target an entry in scope. If a response redirects off-scope (3xx to another host, a link, a CNAME to prod), **STOP** and confirm before following. Full procedure in [`references/scope-enforcement.md`](./references/scope-enforcement.md).

3. **No production without paper.** If the user hasn't said "prod is in scope and I have written sign-off," assume it isn't. Default targets: staging, local docker, a dedicated test instance.

4. **Cloud metadata is off by default.** Do not probe `169.254.169.254`, `metadata.google.internal`, `100.100.100.200`, `[fd00:ec2::254]`, or equivalents unless the engagement explicitly scopes SSRF-to-metadata **and** the target is one you control.

5. **Destructive payloads need approval.** Anything that mutates beyond a single test row — SQLi with `DROP`/`DELETE`, filesystem-write SSTI, command injection with `rm`/`shutdown`/`mkfs` — **ask first**. Prefer non-destructive witnesses (see Phase 4).

6. **Secret-leakage via context compaction.** This work fills the conversation with payloads, captured tokens, and credentials. When Claude Code compacts or summarizes context, that history is replayed through a model — anything sensitive in chat can leave the box. Mitigation:
   - Redact captured tokens/credentials/session cookies to the **last 6 characters** in any chat message (`…a1b2c3`). Full values go only to files under `engagement/evidence/`, never into the conversation.
   - Keep raw request/response dumps in files; reference them by path in chat, don't paste them.

If the task isn't authorized testing — e.g. a target the user doesn't control, or intent to attack a third party — refuse and stop.

---

## Engagement setup (before any scanning)

```bash
mkdir -p engagement/evidence engagement/findings
```

1. **Get authorization in writing.** Ask the user to confirm ownership/authorization and the target(s). Write `engagement/authorization.md` from `templates/authorization.md` — target URLs/IPs, authorization basis, engagement window, out-of-scope items, operator name.
2. **Declare scope.** Write `engagement/scope.txt` — one host/CIDR per line (see `references/scope-enforcement.md` for format). Nothing gets scanned that isn't in this file.
3. Only once both files exist do you proceed to Phase 2.

---

## Phase 1: Pre-Recon (code analysis — optional)

If you have the source (this repo, a given path), read it first — it makes live testing precise instead of blind. Look for: routes/endpoints, input sinks (query builders, template renders, `exec`/`system`, deserialization, file paths), auth/session handling, and the framework's known sharp edges. Write findings to `engagement/evidence/pre-recon.md` as a map of *where to look* live. No live requests happen here.

Skip if the target is a black box.

## Phase 2: Recon (live, read-only)

In-scope, non-mutating requests to map the attack surface. Use `scripts/recon-scan.sh <host>` (rate-limited nmap + whatweb + header/TLS check, scope-gated) or run the pieces manually:

- Tech stack & headers: `whatweb`, `curl -sI`, check for missing security headers.
- Surface: crawl links/forms, enumerate endpoints, find `robots.txt`/`sitemap.xml`, JS-referenced APIs.
- Auth model: cookies, JWTs (decode header/payload — never log the signature in chat), CSRF tokens, CORS.

Write everything to `engagement/evidence/recon.md`. This file is the input to every later phase.

## Phase 3: Vulnerability Analysis (candidate generation)

Turn the recon map into a ranked list of candidates per class, using the taxonomy in [`references/vuln-taxonomy.md`](./references/vuln-taxonomy.md) (slot types, render contexts, OWASP map).

Parallelize with the **Agent tool** — one subagent per vulnerability class (injection, XSS, access-control/IDOR, auth/session, SSRF, misconfig…). Each subagent reads `engagement/evidence/recon.md`, proposes candidate injection points, and writes `engagement/findings/<class>.md` — a queue of `{endpoint, param, hypothesis, why}`. No exploitation yet; this is where to look, not proof.

## Phase 4: Exploitation (proof-based, conditional)

Work each candidate queue. Per candidate, per the loop in [`references/exploitation-techniques.md`](./references/exploitation-techniques.md):

1. **Pre-send check** — host in scope? auth gate satisfied? payload non-destructive (or approved)?
2. **Send a witness payload** — minimal, benign proof. SQLi: `' AND 1=1--` vs `' AND 1=2--`. XSS: a unique marker like `<svg/onload=console.log("PENTEST-XSS-<id>")>` — never `alert(1)` in stored/shared contexts (it fires for real users). Blind: a timing probe (`SLEEP(5)`) and measure. SSRF: a callback host **you control**, not a public collector.
3. **Promote by observed effect:**
   - **L1 Identified** — pattern matched, no behavior change
   - **L2 Partial** — sink reached but a defense held
   - **L3 Confirmed** — payload changed app behavior observably
   - **L4 Critical** — data extracted, code executed, access escalated
4. **Bypass exhaustion before FP.** If a candidate blocks, try at least the class's set in [`references/bypass-techniques.md`](./references/bypass-techniques.md) before writing `verdict: false_positive`.
5. **Record evidence** for every L3/L4 in `engagement/evidence/<finding>.md`: full request (method/URL/headers/body), response (status/headers/relevant excerpt), a `curl` reproducer, and an impact statement. Secrets redacted to last-6 in chat, full in the file.

## Phase 5: Reporting

Generate the final report from `templates/pentest-report.md`:

1. Executive summary
2. Scope (from `engagement/scope.txt`)
3. Authorization (from `engagement/authorization.md`)
4. **Findings (L3/L4 only — proof required)** — per finding: title, severity (CVSS 3.1), CWE, affected endpoint(s), proof (request + response excerpt), reproduction steps, impact, remediation
5. Not-exploited candidates (L1/L2, with what blocked them)
6. Out-of-scope observations
7. Methodology / tools used
8. Limitations — what was **not** tested

---

## Files in this skill

- [`references/scope-enforcement.md`](./references/scope-enforcement.md) — how to bound every active request; host-extraction rules; the pre-send checklist.
- [`references/vuln-taxonomy.md`](./references/vuln-taxonomy.md) — vulnerability classes, injection contexts, OWASP Top 10 map.
- [`references/exploitation-techniques.md`](./references/exploitation-techniques.md) — the per-candidate witness loop and per-class payload patterns.
- [`references/bypass-techniques.md`](./references/bypass-techniques.md) — WAF/filter bypass sets to exhaust before dismissing.
- [`templates/authorization.md`](./templates/authorization.md) — engagement authorization record.
- [`templates/pentest-report.md`](./templates/pentest-report.md) — final report.
- [`scripts/recon-scan.sh`](./scripts/recon-scan.sh) — scope-gated, rate-limited recon wrapper.

Credit: methodology adapted from Nous Research's `web-pentest` skill (Hermes Agent) and the "No Exploit, No Report" discipline; payload content is standard OWASP material.
