# Anti-slop (subskill of design-taste)

Load this before shipping, or when a design "looks AI-made / templated / generic." This is the enforcement arm of the one rule — *never ship the default*. It names the tells and gives the pre-flight gate.

## The tells (recognizable AI/template defaults)

If you spot these, the design was skipped, not made:

- **Default framework palette** — indigo/violet-500 buttons, `gray-50` background, straight off Tailwind/Bootstrap.
- **Purple-to-blue gradient hero** — the single most overused AI hero.
- **System-font-only by accident** — no chosen typeface, default tracking, one weight.
- **Everything centered** — centered hero, centered body text, centered three-card row.
- **Three equal cards, no hierarchy** — features as identical boxes, nothing leading.
- **Uniform 8px spacing** — no rhythm, no grouping, no scale.
- **Emoji as icons** — 🚀✨🔥 standing in for a real icon set.
- **Decorative accent bars / colored edges** — a colored left-border stripe on every card, callout, or blockquote (`border-left: 4px solid indigo`); a gradient/colored top-bar on sections; glowing colored borders. A colored edge is not hierarchy — it's decoration standing in for a decision, and it screams AI. Define edges with hairline neutral borders or elevation; carry meaning through type, weight, and space, not a stripe.
- **One-layer hard drop shadow** — `0 4px 6px rgba(0,0,0,0.1)` on everything.
- **Unmodified component-library look** — the page is recognizably "shadcn defaults" / "MUI defaults" with no direction applied.
- **Pure `#000` on `#fff`** — no off-black, no palette temperature.
- **Lorem-density filler** — every section the same length, no editorial hierarchy.
- **No designed states** — flat hovers, missing focus rings, no empty/loading states.

## Pre-flight checklist

Answer each. If any answer is "default / no / not sure," you're not done — go back to that reference.

- [ ] **Direction** — can you name the one direction in a word, and 2–3 reference sites? (`direction.md`)
- [ ] **Type** — real face(s), a modular scale (not ad-hoc sizes), headings tightened, measure ≤ 75ch? (`typography.md`)
- [ ] **Color** — a chosen ramp + accent (not framework defaults), off-black/near-white, contrast passes AA? (`color.md`)
- [ ] **Space** — every value on one spacing scale, whitespace generous, not everything centered? (`layout.md`)
- [ ] **Hierarchy** — is exactly one thing clearly the hero? Passes the squint test at 50% zoom?
- [ ] **Polish** — consistent radii, layered soft shadows, all four interaction states, focus-visible rings, reduced-motion honored? (`polish.md`)
- [ ] **Intent** — could you defend every visible token as a decision that serves the direction, not a leftover?

## The squint test

Zoom to 50% (or blur your eyes). If the visual priority is obvious and the page still reads as *this specific direction* — not generic — it passes. If it dissolves into an even gray mush of same-sized boxes, hierarchy failed.

## When you find slop

Don't patch one symptom. Trace it to the skipped step: default colors → no palette was built (`color.md`); flat mush → no hierarchy (`layout.md` + type scale); cheap feel → no polish pass (`polish.md`). Fix the system, and the symptoms across the whole page resolve at once.
