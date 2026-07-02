# Layout & space (subskill of design-taste)

Load this when laying out a page or when spacing feels arbitrary. Consistent space is invisible when right and glaring when wrong — most "amateur" layouts are just inconsistent spacing.

## A spacing scale, not random numbers

Every margin, padding, and gap comes from one scale. Base on 4px; step non-linearly so the jumps are meaningful:

```
4 · 8 · 12 · 16 · 24 · 32 · 48 · 64 · 96 · 128
```

Ban off-scale values (`padding: 13px`, `margin: 27px`). If you need a value that isn't on the scale, the scale is wrong — not the layout.

## Rhythm

- Space **between** related items is smaller than space **around** the group (proximity = grouping). Label→input is 8px; field→field is 24px; section→section is 96px.
- Vertical rhythm: section padding scales with importance — a hero breathes (`padding: 96–128px` block), a dense table doesn't.

## Measure & content width

- Cap reading columns at **~65ch**; cap full page content at **~1100–1280px** with auto margins. Full-bleed backgrounds, contained content.
- Text that runs the full width of a 1440px monitor is unreadable — this is a top layout tell.

## Grid

- Marketing/editorial: a **12-column** grid you actually use for asymmetry (7/5 splits, offset images) — not everything centered.
- App/dashboard: a sidebar + content grid, or CSS grid with named areas. Keep gutters on the spacing scale.
- `display: grid` with `gap` beats manual margins for anything repeating (card rows, feature grids).

## Whitespace is a feature, not waste

Generous negative space is what makes work read as premium and confident. When in doubt, add space rather than fill it. Cramped = cheap; the fix for "looks busy" is almost always more space, not smaller elements.

## Density by context

Match the direction (see `direction.md`): marketing is loose and airy; a dashboard or data tool is deliberately tight so more fits on screen. Don't apply marketing spacing to a data grid or vice-versa.

## Alignment

- Pick a small number of alignment lines and snap everything to them. Ragged left edges are the fastest way to look sloppy.
- Center sparingly — centered body text and centered everything is a slop tell. Left-align text; reserve centering for short hero lines and truly symmetric layouts.

## Responsive

- Mobile-first, few breakpoints (~640 / 1024). Don't design 6 breakpoints.
- Scale the *spacing scale itself* down on mobile (section padding 96→48), don't just reflow.

## Anti-patterns

- Everything at 8px / everything centered / three equal cards with no hierarchy.
- Off-scale one-off spacing values.
- Content stretched to full viewport width.
- Uniform spacing that ignores grouping (proximity carries meaning — use it).
