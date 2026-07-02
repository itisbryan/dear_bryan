# Typography (subskill of design-taste)

Load this when choosing fonts or when the type feels flat. Type is ~80% of what makes web design read as tasteful — most "generic" designs are generic *because* the type was never designed.

## 1. Pick faces that carry the direction

- **One superfamily** (Inter, Geist, IBM Plex, Söhne-likes) covers most product/technical work — sans + mono in one system.
- **A pairing** for editorial/luxe: an expressive display face for headings + a quiet workhorse for body (e.g. a high-contrast serif display + a neutral sans body). Never pair two faces that do the same job.
- **System-font-only is a decision, not a default** — valid for deliberately utilitarian/native UIs, wrong when reached for by omission.

Load fonts with the actual weights you use (`wght@400;500;600;700`), and include a mono if you show code, data, or technical labels.

## 2. A modular type scale

Don't pick sizes ad hoc. Choose a ratio and step from a base.

- Base body: **16px** (never below 14px for reading text).
- Ratio: **1.2** (dense/UI) to **1.333** (marketing/editorial).
- Example (1.25, base 16): 12 · 14 · 16 · 20 · 25 · 31 · 39 · 49 · 61px. Round to sensible values.

Use maybe 5–6 sizes total across the whole page. More sizes = less hierarchy.

## 3. Tracking (letter-spacing) by size

The detail that separates designed type from default:
- **Large headings** — tighten: `-0.02em` to `-0.04em` (bigger = tighter).
- **Body** — leave at `0` (the font is tuned for it).
- **Small caps / labels / eyebrows** — open up: `+0.04em` to `+0.1em`, often `text-transform: uppercase`.

## 4. Line-height by role

- Display / large headings: **1.05–1.15** (tight, they're already big).
- Body: **1.5–1.65** (comfortable reading).
- UI / dense labels: **1.2–1.35**.

## 5. Measure (line length)

Body text: **60–75 characters** per line (`max-width: 65ch`). Wider is unreadable; narrower is choppy. This single constraint fixes most "wall of text" pages.

## 6. Weight contrast

Use real weight jumps to build hierarchy: e.g. body **400**, headings **600–700**, labels **500**. A page set entirely in one weight reads flat — weight is free hierarchy.

## 7. The refinements

- `font-feature-settings: "liga" 1;` — enable ligatures.
- Tables/data: `font-variant-numeric: tabular-nums;` so digits align.
- Don't force `-webkit-font-smoothing` unless the direction demands it; leave default on the platform it renders.

## Quick token block

```css
:root {
  --font-sans: 'Inter', system-ui, -apple-system, sans-serif;
  --font-mono: 'Geist Mono', ui-monospace, monospace;
  --step--1: 0.833rem;  --step-0: 1rem;    --step-1: 1.25rem;
  --step-2: 1.563rem;   --step-3: 1.953rem; --step-4: 2.441rem; --step-5: 3.052rem;
  --lh-tight: 1.1; --lh-body: 1.6;
}
h1 { font-size: var(--step-5); line-height: var(--lh-tight); letter-spacing: -0.03em; font-weight: 700; }
p  { font-size: var(--step-0); line-height: var(--lh-body); max-width: 65ch; }
```

## Anti-patterns

- One size, one weight, default tracking → flat and templated.
- 9 different font sizes → no hierarchy at all.
- Headings not tightened; labels not opened → the "untouched font" look.
