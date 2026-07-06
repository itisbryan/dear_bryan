# Engagement Authorization

> Fill this out and get the operator's written acknowledgement **before** any active scanning. This file is a record, not a substitute for a real contract/SOW where one applies.

## Targets

- **In-scope URL(s):**
- **In-scope IP(s) / CIDR(s):** (must match `engagement/scope.txt`)

## Authorization basis

- [ ] I **own** the target system, or
- [ ] I have **written authorization** from: `<name / org>` (attach/reference the sign-off)

## Engagement window

- **Start:**
- **End:**
- Testing outside this window is not authorized.

## Out of scope (do NOT test)

- Production systems (unless explicitly listed as in-scope below)
- Third-party services, SaaS, payment processors, CDNs
- Cloud metadata endpoints (unless SSRF-to-metadata is an explicit objective on owned infra)
- Denial-of-service / load testing
- Social engineering / physical
- Other: `…`

## Rules of engagement

- Non-destructive by default. Destructive payloads require explicit per-case approval.
- Findings and captured secrets stay in `engagement/`; nothing is disclosed to third parties.
- Stop-condition: if the target shows signs of real user impact or instability, halt and notify.

## Operator

- **Name:**
- **Contact:**
- **Acknowledged (operator confirms the above is accurate and authorized):** `<yes + date>`
