# Polish (subskill of design-taste)

Load this when something's "off" or looks cheap despite being technically correct. The distance between "fine" and "expensive" is entirely micro-detail — these are the finishing moves.

## Borders

- Hairlines, not heavy lines: `1px solid` at low-alpha (`rgba(0,0,0,0.08)`), never `#000`.
- **Shadow-as-border** (the Vercel/Linear technique): `box-shadow: 0 0 0 1px rgba(0,0,0,0.08)` gives a border in the shadow layer — crisper corners, composites with elevation shadows, no box-model shift on hover.
- **No decorative accent bars or colored edges.** A border defines an edge; it does not decorate one. No colored left-stripe on cards/callouts/blockquotes, no gradient top-bar, no glowing colored border — those are an AI slop tell (see `anti-slop.md`). If a callout needs to signal type (info/warn), carry it with a small icon + text color, not a colored bar. Borders stay neutral hairlines.

## Radius scale

Consistent, not random. Pick a small ramp and apply by element size:

```
--r-sm: 6px;   /* inputs, chips, small buttons */
--r-md: 10px;  /* cards, panels */
--r-lg: 16px;  /* modals, large surfaces */
--r-full: 9999px; /* pills, avatars */
```

Nested radii: inner radius = outer − padding, so corners stay concentric.

## Shadows — layered and soft

Default `box-shadow: 0 2px 4px rgba(0,0,0,0.5)` is the tell. Real elevation stacks 2–3 low-alpha layers:

```css
--shadow-sm: 0 1px 2px rgba(0,0,0,0.04), 0 1px 1px rgba(0,0,0,0.06);
--shadow-md: 0 4px 8px rgba(0,0,0,0.04), 0 2px 4px rgba(0,0,0,0.06), 0 0 0 1px rgba(0,0,0,0.04);
--shadow-lg: 0 12px 24px rgba(0,0,0,0.08), 0 4px 8px rgba(0,0,0,0.04);
```

Shadows are **never pure black** and the offset grows with elevation. On dark backgrounds shadows barely read — lean on lighter surfaces for elevation instead.

## Designed states (all of them)

Every interactive element needs four states designed, not defaulted:
- **hover** — a small shift: background tint, `translateY(-1px)`, or shadow bump.
- **focus-visible** — a real ring (`outline: 2px solid var(--accent); outline-offset: 2px`). Never `outline: none` with no replacement — that's an accessibility break, not a style choice.
- **active** — press feedback (`translateY(0)`, slightly darker).
- **disabled** — reduced opacity + `cursor: not-allowed`.

## Motion — restrained

- Duration **150–250ms**, ease-out (`cubic-bezier(0.2,0,0,1)` or `ease-out`).
- Animate **`transform` and `opacity` only** (GPU-cheap, no layout thrash). Not `width`/`top`/`margin`.
- Stagger list entrances ~30–50ms apart for a hand-made feel.
- Always: `@media (prefers-reduced-motion: reduce) { *, ::before, ::after { animation: none !important; transition: none !important; } }`

## Optical alignment

Trust the eye over the math: icons next to text often need a 1px nudge; a triangle "play" button centers off-center; large quotes/punctuation hang into the margin. Mathematically centered ≠ looks centered.

## Image & media treatment

- Give images a subtle containing ring: `box-shadow: inset 0 0 0 1px rgba(0,0,0,0.06)` — stops light images bleeding into a light background.
- Consistent aspect ratios (`aspect-ratio: 16/9`) so grids don't jump.

## Details that quietly signal care

- `tabular-nums` on any changing/aligned numbers.
- Consistent icon stroke width and size across the whole UI.
- Text never touches an edge — always padding.
- Loading/empty/error states designed, not blank.

## The tell list (kill these)

- One-layer hard black shadow · sharp mixed radii · `outline: none` with no focus style · hover with no transition · `transition: all` · animating layout properties · icons at mismatched weights.
