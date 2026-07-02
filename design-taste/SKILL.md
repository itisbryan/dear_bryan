---
name: design-taste
description: Guides distinctive, intentional web/UI design — recommends or lets you pick a direction, or mirrors a specific site ("make it look like Stripe / Linear / this URL"), then builds a real type/color/space system so the result looks decided-on, not defaulted-into. Loads focused references (catalog, mirror, typography, color, layout, polish, anti-slop) on demand. Use when building or restyling a landing page, dashboard, portfolio, or any UI, or when a design "looks generic / templated / like AI made it."
---

# Design Taste

Helps you design interfaces that look **decided-on, not defaulted-into**. This skill is about the taste and the process — inferring the right direction for a brief, committing to a real system, and refusing the defaults that make work read as templated. It supplies judgment, not a component library.

## The one rule that overrides everything

**Never ship the default.**

Every visible choice is a decision, not a framework leftover:
- Not untouched Tailwind gray + indigo-500 buttons. Not system-font-only by accident. Not 8px everywhere, everything centered, three equal cards in a row.
- If a token (font, size, color, radius, shadow, spacing) is still at its out-of-the-box value, you haven't designed it — you've skipped it.
- A design with no clear hierarchy, no committed direction, and default components is the thing this skill exists to prevent.

When unsure, commit to a direction first and let it decide the tokens. A direction is always safe; a pile of defaults never is. See [`references/anti-slop.md`](./references/anti-slop.md) for the specific tells and the pre-flight check.

## The process

### 0. Audit first (if something already exists)

Restyling, not greenfield? Read the current UI before touching it. Name what's already working (keep it), what reads as default/templated (the target), and the constraints you can't break (existing tokens, brand, framework). Don't redesign what's fine.

### 1. Pick a direction — recommend, or let them choose

Every design commits to **one** direction. Three ways to get there:

- **Recommend (default):** inspect the current project — product type, audience, existing brand colors/fonts, framework, current vibe — and recommend the direction that fits, with the reasoning. Use the rubric in [`references/catalog.md`](./references/catalog.md). Say *why* it fits ("dev tool + technical audience → Technical/utilitarian, like Linear").
- **Choose:** present the catalog menu and let the user pick. Honor an explicit pick even if you'd have recommended otherwise — but flag a genuine mismatch once ("Brutalist on a banking app will read as untrustworthy — sure?").
- **Mirror a specific site:** the user names a brand or pastes a URL ("make it look like Stripe," "mirror this page"). Load [`references/mirror.md`](./references/mirror.md) — extract its system (signature move, fonts, palette, shape, spacing) and adopt it as the direction. For a live URL, inspect the real page rather than guessing.

Either way, lock the three artifacts before building: the named direction, 2–3 real reference sites, and one adjective triad ("precise, quiet, engineered"). The deeper *how to commit* guidance is in [`references/direction.md`](./references/direction.md). This is the highest-leverage step and the most-skipped.

### 2. Build the system before the pixels

Establish the tokens the direction implies — type scale, palette, spacing scale — *before* laying out any component. A page assembled from designed tokens is coherent; a page where each element is styled ad hoc is slop. Load the relevant reference:
- Type → [`references/typography.md`](./references/typography.md)
- Color → [`references/color.md`](./references/color.md)
- Space & layout → [`references/layout.md`](./references/layout.md)

### 3. Compose with hierarchy

Lay out from the tokens. One thing is clearly the hero; everything else is subordinate through size, weight, color, and space — not decoration. Squint test: at 50% zoom, is the visual priority obvious?

### 4. Polish the details

The gap between "fine" and "expensive" is micro-detail: hairline borders, a consistent radius scale, layered soft shadows, designed hover/focus/active states, restrained motion. Load [`references/polish.md`](./references/polish.md).

### 5. Pre-flight before you ship

Run the checklist in [`references/anti-slop.md`](./references/anti-slop.md). If any answer is "default," you're not done.

## References (load on demand)

Load only on the trigger, not speculatively — progressive disclosure is the point. Each file carries concrete values (real sizes, ratios, hex, ms), the same spirit as this skill's one rule.

| Trigger | Load | What it does |
|---|---|---|
| Step 1 — user wants to pick a look or you need to recommend one | [`references/catalog.md`](./references/catalog.md) | The pickable direction menu + a rubric to recommend the one that fits the project. Each entry has starter tokens. |
| Step 1 — user wants to look like a specific site ("like Stripe", a URL) | [`references/mirror.md`](./references/mirror.md) | Extract a real site's design system; brand cheat-sheet + method for any URL. |
| The direction is chosen but you need to commit & lock it well | [`references/direction.md`](./references/direction.md) | How to commit to one direction, pick references, and lock an adjective triad. |
| Choosing fonts, sizes, or the type feels flat | [`references/typography.md`](./references/typography.md) | Build a real type system: superfamily/pairing, modular scale, tracking, measure, weights. |
| Building a palette, or colors look like defaults | [`references/color.md`](./references/color.md) | Neutral ramp + accent, off-black/near-white, contrast (AA), real dark mode. |
| Laying out a page, or spacing feels arbitrary | [`references/layout.md`](./references/layout.md) | Spacing scale, rhythm, measure, grid, whitespace, density by context. |
| Something's "off" or looks cheap despite being correct | [`references/polish.md`](./references/polish.md) | Borders, radii, layered shadows, motion, designed states, optical alignment. |
| Before shipping, or a design "looks AI-made / templated" | [`references/anti-slop.md`](./references/anti-slop.md) | The default tells to kill + the pre-flight checklist. |

Rules for loading references:
- Load only when its trigger fires. Follow that file's guidance fully once loaded.
- A reference is part of this skill, not a separate skill — don't announce a skill switch; just use it.

## Tone

- Opinionated but not precious. Recommend a direction; don't survey ten.
- Concrete over vague. "Tighten the h1 to -0.03em" beats "make it feel tighter."
- Show the reasoning so the taste transfers: name *why* a choice fits the direction.

## When NOT to use this skill

- The user wants raw feature/logic code with no visual surface → normal coding workflow.
- A design system / brand tokens are already mandated and fixed → apply them, don't re-derive.
- The user wants to *learn* design by doing rather than get output → use the `tutor` skill.
- Deep accessibility audit or interaction engineering beyond visual polish → out of scope; hand off.
