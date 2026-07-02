# Color (subskill of design-taste)

Load this when building a palette or when colors look like defaults. The default Tailwind gray + indigo-500 is the single most recognizable AI/template tell — a real palette kills it instantly.

## The structure of a real palette

Most tasteful UIs run on **one neutral ramp + one accent** (occasionally a second). Not a rainbow.

1. **Neutral ramp** — 6–9 steps from background to text. Pick a *temperature* and hold it: warm neutrals (slight yellow/red) feel human; cool neutrals (slight blue) feel technical. Never mix warm and cool grays.
2. **One accent** — the direction's signature color, used sparingly (links, primary action, focus). If everything is accent-colored, nothing is.
3. **State colors** — success / warning / danger, muted to match the palette's saturation (not raw `#00ff00`).

## Never pure black or pure white

- Text: **off-black** — `#171717`, `#1a1a1a`, or a very dark tinted neutral. Pure `#000` on `#fff` vibrates and feels cheap.
- Background: **near-white** with the ramp's temperature — `#fafafa`, `#f8f7f5` (warm). Or an off-black canvas for dark directions.

## Contrast is non-negotiable

- Body text: **≥ 4.5:1** against its background (WCAG AA).
- Large text (≥24px) and UI/icons/borders: **≥ 3:1**.
- Check the accent-on-background pair actually passes — many brand accents fail on white and need a darker variant for text.

## Saturation and harmony

- Keep saturation consistent across the palette — one vivid color among muted ones looks like an accident (unless that's the point).
- Desaturate slightly as colors get lighter/darker so the ramp feels natural, not linearly interpolated.

## Real dark mode (not an invert)

Inverting light mode is the dark-mode tell. Instead:
- **Lift surfaces with elevation, not borders** — raised panels get a lighter background (`#1a1a1a` → `#242424`), the way real light works.
- **Desaturate accents** — vivid colors glare on dark; pull them down ~10–20%.
- Text is off-white (`#ededed`), not pure white; borders become subtle light-alpha (`rgba(255,255,255,0.08)`).

## Quick token block

```css
:root {
  --bg: #fafafa;      --surface: #ffffff;   --border: rgba(0,0,0,0.08);
  --text: #171717;    --text-muted: #6b6b6b;
  --accent: #533afd;  --accent-ink: #ffffff;
  --success: #2f855a; --warning: #b7791f;  --danger: #c53030;
}
@media (prefers-color-scheme: dark) {
  :root {
    --bg: #0a0a0a;    --surface: #171717;   --border: rgba(255,255,255,0.09);
    --text: #ededed;  --text-muted: #a1a1a1;
    --accent: #7c6cff; --accent-ink: #0a0a0a;
  }
}
```

## Anti-patterns

- Default framework palette untouched (indigo-500 button on gray-50).
- Pure `#000` / `#fff`.
- Three+ competing accent colors with no hierarchy.
- Dark mode that's a straight invert with the same saturated accents.
