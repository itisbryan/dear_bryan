# Pixel-perfect clone (subskill of design-taste)

Load this when the user wants an **exact rebuild** of a real site — "clone this," "pixel-perfect," "rebuild this page 1:1" — not just its vibe. This is the high-fidelity sibling of [`mirror.md`](./mirror.md): mirror adopts a site's *system* and adapts it to your content; clone reproduces the actual page. Choose clone only when faithfulness to the original is the goal.

## What you need

Browser automation with **JS evaluation** — navigate, set viewport, screenshot (full-page and element), scroll, click, and run scripts in the page (`getComputedStyle`, DOM queries). In Claude Code that's the Playwright MCP browser tools; elsewhere it's `agent-browser` or any equivalent. Written against those capabilities, not one tool's names.

## The one rule

**Completeness beats speed — never guess a value.** Every element you rebuild must come from an *extracted* exact value: the real hex, the real px, the real font, the downloaded asset, the real text. If a builder step has to approximate a color, size, or spacing, extraction failed — go back and pull the real value. A minute more extracting beats an hour of "close enough but clearly wrong."

Second rule from the same principle: **small tasks, perfect results.** "Rebuild the whole page" makes you gloss and approximate; one section at a time, from an exact spec, nails it.

## The workflow — walk the page like a foreman

### Phase 1 — Recon & assets

- Full-page screenshots at **desktop (1440×900)** and **mobile (390×844)** for reference.
- Inventory the globals: fonts actually in use, the color palette, favicon.
- **Download every asset** to a local folder at natural resolution — images, videos, background-images, SVGs, fonts. Note **layered/stacked images** (multiple `<img>` or background layers in one container) and their `z-index`. Assets you rebuild from memory are assets you get wrong.

Asset inventory (run via your browser's JS-eval):

```js
JSON.stringify({
  images: [...document.querySelectorAll('img')].map(img => ({
    src: img.currentSrc || img.src, alt: img.alt,
    w: img.naturalWidth, h: img.naturalHeight,
    parent: img.parentElement?.className,
    layered: img.parentElement ? [...img.parentElement.querySelectorAll('img')].length : 1,
    position: getComputedStyle(img).position, zIndex: getComputedStyle(img).zIndex,
  })),
  backgroundImages: [...document.querySelectorAll('*')]
    .filter(el => getComputedStyle(el).backgroundImage !== 'none')
    .map(el => ({ url: getComputedStyle(el).backgroundImage, el: el.tagName + '.' + (el.className?.split?.(' ')[0] || '') })),
  fonts: [...new Set([...document.querySelectorAll('*')].slice(0, 300).map(el => getComputedStyle(el).fontFamily))],
  favicons: [...document.querySelectorAll("link[rel*=icon]")].map(l => l.href),
}, null, 2)
```

### Phase 2 — Section by section

Split the page into sections (nav, hero, features, testimonials, footer…). For **each** section:

1. **Screenshot it in isolation** — scroll to it, capture the viewport.
2. **Extract exact computed styles** for its key elements (see snippet below) — layout, box model, type, color, shadow, radius, transform.
3. **Capture the real text content** verbatim.
4. **Identify its assets** from the Phase-1 inventory (which images/SVGs, layered or not).
5. **Note responsive + interactive states** — what changes at the mobile breakpoint, and hover/focus/scroll states (re-extract styles in the hovered/scrolled state to capture them).
6. **Write a spec file** for the section — this is the contract for the build. It must contain: screenshot path, exact CSS values, asset paths, verbatim text, DOM/component structure, and breakpoint behavior. Keep it as an auditable artifact so drift is traceable.

Exact computed styles for one element:

```js
(sel => {
  const el = document.querySelector(sel), s = getComputedStyle(el), r = el.getBoundingClientRect();
  const keys = ['display','position','flexDirection','justifyContent','alignItems','gap','gridTemplateColumns',
    'width','height','maxWidth','padding','margin','fontFamily','fontSize','fontWeight','lineHeight',
    'letterSpacing','textAlign','color','backgroundColor','backgroundImage','border','borderRadius',
    'boxShadow','opacity','transform','transition','zIndex'];
  return JSON.stringify({ rect: { w: Math.round(r.width), h: Math.round(r.height) },
    styles: Object.fromEntries(keys.map(k => [k, s[k]])) }, null, 2);
})('SELECTOR')  // e.g. 'header .cta', '.hero h1'
```

### Phase 3 — Build from the specs

Build each section **from its spec, never from memory** — paste exact values, wire the downloaded local assets, use the captured text. If you dispatch parallel builder agents, each must receive its **full spec inline** (not "go read the file") plus the section screenshot, and write to an isolated location to avoid conflicts. Match values exactly; a `padding: 24px` spec becomes `24px`, not "about 1.5rem."

### Phase 4 — Verify by comparison (the loop)

Extraction and building aren't done until the clone matches:

1. Serve your clone; screenshot it at the **same viewports** (1440×900 and 390×844).
2. Put clone beside original and compare — hunt drift in spacing, color, font, alignment, asset placement.
3. Fix the drift and re-compare. **Iterate until it matches** — never declare done off a single build. This comparison loop is where "close enough" becomes correct.

## Boundary (read before cloning)

Pixel-perfect cloning reproduces someone else's work — treat it as a trust boundary, not a free-for-all:

- Fine: cloning a site **you own** or are **authorized to rebuild**, a redesign engagement, or a private learning exercise.
- Not fine: passing off another brand's site/content as your own, cloning to deceive users, or lifting trademarked logos, copy, imagery, and brand marks into a public product. The layout and CSS techniques are learnable; the *content and brand assets* are the owner's.
- When intent is unclear, ask what the clone is for before downloading a full site.

If the user only wants the *feel* of the site for their own content, that's `mirror.md`, not this — it's lighter, and it sidesteps the content-ownership problem entirely.
