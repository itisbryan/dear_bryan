# QA Sweep Report

**Target:** {target_url}
**Date:** {date}
**Scope:** {scope_description}
**Tester:** qa-sweep (automated exploratory QA)

---

## Executive summary

| Severity | Count |
|----------|-------|
| 🔴 Critical | {critical_count} |
| 🟠 High | {high_count} |
| 🟡 Medium | {medium_count} |
| 🔵 Low | {low_count} |
| **Total** | **{total_count}** |

**Overall assessment:** {one_sentence_verdict — is it shippable, and what's the headline risk?}

**Areas needing deeper testing:** {anything covered only shallowly, or not reached}

---

## Findings

Sorted Critical → Low. One block per finding.

### {N}. {short title} — {🔴/🟠/🟡/🔵} {Severity} · {Category}

- **Where:** {page/URL/component}
- **Steps to reproduce:**
  1. {step}
  2. {step}
- **Observed:** {what actually happened}
- **Expected:** {what should have happened}
- **Console:** {verbatim errors, or "none"}
- **Evidence:** `screenshots/{filename}.png`
- **Confirmed:** {yes / unconfirmed — couldn't reliably reproduce}

---

## Not covered

- {pages/flows out of scope or blocked, and why — so the reader knows the gaps}
