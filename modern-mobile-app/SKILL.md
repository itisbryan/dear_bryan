---
name: modern-mobile-app
description: Design modern mobile app UI/UX and engagement using evidence-based patterns from 500+ studied apps — gamification mechanics that actually work, the 3-stage reward ceremony, plus Mobbin-channel findings on onboarding, paywalls, dashboards, streaks, and A/B-tested monetization. Refuses PBL theater, streak fear, and ceremony-as-decoration. Use when designing or auditing mobile app engagement, retention loops, reward moments, onboarding, paywalls, or dashboards.
---

# Modern Mobile App Design

Ships mobile experiences that feel designed, not defaulted-into — backed by what actually worked across 500+ apps (Tim Gabe's studies) and the Mobbin channel's pattern studies (1,460 onboarding flows, 2,995 paywalls, 2,108 dashboards, 859 streak designs, $1M of A/B tests). Two rails: **engagement mechanics** (what drives behavior) and **reception ceremony** (how the user receives it).

## The one rule that overrides everything

**Design for the behavior, not the metric.**

Every mechanic and every moment of ceremony must move a real-world outcome — the thing the user does off-screen. Points/badges/leaderboards that only inflate app-opens are decoration (Foursquare, Google News, LinkedIn all killed theirs). Confetti that trivializes high-stakes actions gets you regulated (Robinhood: $7.5M fine, celebratory imagery banned). If a pattern can't name the behavior it serves, it ships nowhere.

## The process

### 1. Name the outcome, list the receive-moments

Outcome: one sentence, the verb, the real world. Then inventory every moment the user *receives* something — matches, scores, reports, milestones, streaks, earnings, completions — and label each a **receipt** (flat, informational) or a **gift** (ceremonial, felt). The receipts are your targets.

If the surface is onboarding, a paywall, a dashboard/home screen, or a streak system, load the matching reference first — it carries the channel's hard-won specifics (see the table below).

### 2. Choose the engagement stack (at most 3 mechanics)

From [`references/principles.md`](./references/principles.md), in preference order: outcome scoreboard → small winnable competition → social acknowledgment → variable pull rewards → competence signaling. Never start from points/badges/leaderboards.

### 3. Ceremonialize the receive-moments (the 3-stage trick)

From [`references/reward-ceremony.md`](./references/reward-ceremony.md): turn each target receipt into a gift via **Anticipation → Reveal → Celebration**. Build uncertainty before the reveal (even seconds of build-up rewires the response), deliver sequentially — never in bulk — and close with an afterglow artifact (share prompt, badge, screenshot-able stat) so the moment becomes identity. Ceremony is behavior design, not decoration: never use it to nudge harmful or high-stakes actions.

### 4. Run the anti-pattern check

Every mechanic and every ceremony against the checks in all loaded references: PBL theater, streak fear, S-curve bloat, game-layer absorption, decoration-only ceremony, hidden paywalls, correlation-as-causation copying. One failed check kills it or forces a redesign. No exceptions.

### 5. Pre-ship the checklists

Run the combined pass/fail checks in [`references/principles.md`](./references/principles.md) and [`references/reward-ceremony.md`](./references/reward-ceremony.md), plus any checks in the surface-specific references you loaded. A single fail is a ship-blocker.

## References (load on demand)

Load only on the trigger, not speculatively — progressive disclosure is the point.

| Trigger | Load | What it does |
|---|---|---|
| Step 2 — choosing mechanics, or you need the evidence behind one | [`references/principles.md`](./references/principles.md) | The mechanics menu, the anti-patterns, the evidence (apps + studies), the pre-ship checks. |
| Step 3 — ceremonializing a receive-moment | [`references/reward-ceremony.md`](./references/reward-ceremony.md) | The 3-stage sequence (anticipation → reveal → celebration), rules of thumb, ethical limits, worked examples. |
| Designing or auditing onboarding | [`references/onboarding.md`](./references/onboarding.md) | 1,460 flows: conversation over form, personalize-before-monetize, product taste first, the right-moment asks. |
| Designing or auditing paywalls | [`references/paywalls.md`](./references/paywalls.md) | 2,995 paywalls + 4,700 experiments: paywall as flow, risk reduction, experiment order (design → personalization → price). |
| Designing or auditing dashboards / home screens | [`references/dashboards.md`](./references/dashboards.md) | 2,108 dashboards: verdict-first, anticipate the emotion, coin your metric, restraint. |
| Designing streaks or habit loops | [`references/streaks.md`](./references/streaks.md) | 859 streak designs: fear vs. identity, design-for-the-miss, why freezes exist. |
| Monetization, pricing, or experimentation strategy | [`references/testing.md`](./references/testing.md) | Moonly's $1M in A/B tests: test everything, end-to-end message matching, audience-mirroring imagery, friction audits. |
| Designing from shipped references (especially with AI) | [`references/research-method.md`](./references/research-method.md) | Research-then-design workflow: category patterns, screen-level teardowns, copy as a variable, traceable decisions. |
| A pattern's evidence is thin or the case is novel | All loaded | Apply the one rule and the anti-pattern checks as first principles — say the evidence doesn't cover it rather than inventing support. |

Rules for loading references:
- A reference is part of this skill, not a separate skill — don't announce a skill switch; just use it.

## Tone

- Opinionated but not precious. Recommend the stack; don't survey ten.
- Concrete over vague. "Three daily targets that reset at midnight, each with a visible fill state" beats "a progress system."
- Show the reasoning so the judgment transfers: name *why* a pattern fits the outcome.

## When NOT to use this skill

- Raw feature/logic code with no engagement surface → normal coding workflow.
- Mechanics already mandated by the product → audit them against the anti-patterns, don't re-derive the stack.
- The user wants to *learn* the psychology by doing rather than get output → use the `tutor` skill.
- Visual design system work with no behavioral surface → `design-taste` or `macos-app-design`.
