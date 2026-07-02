# Mirror a real design (subskill of design-taste)

Load this when the user wants to look like a specific site — "make it look like Stripe," "design like Linear," "mirror this page" + URL. Mirroring is a shortcut to a direction: instead of composing one, you adopt a proven system. The one rule still holds — you capture a *system*, you don't ship the source's defaults or its assets.

## Two sources

- **Known brand** (Stripe, Linear, Vercel…) → use the cheat-sheet below plus what you know of the site, then **verify against the live site** — tokens drift as brands redesign.
- **A URL the user pastes** → inspect the real page (fetch it / screenshot it) and extract tokens from the actual CSS. Never guess when you can read the source.

## What to extract (the system, not the pixels)

Pull these six, in order — they *are* the design system:

1. **Signature move** — the one thing that makes it recognizable (Vercel's shadow-as-border, Stripe's fluid gradient, Linear's dark glow, Apple's cinematic whitespace). Get this right and 60% of the resemblance lands.
2. **Fonts** — the real face + a CDN substitute if it's proprietary (see substitutes below), with the actual weights and tracking.
3. **Palette** — background, text, muted text, accent, borders. Exact hex.
4. **Type scale & tracking** — base size, ratio, and the display letter-spacing.
5. **Shape & depth** — radius scale, border style, shadow philosophy.
6. **Spacing & density** — how airy/tight, section rhythm.

Feed these into `typography.md` / `color.md` / `layout.md` / `polish.md` exactly as a chosen direction — mirroring replaces step 1, not steps 2–4.

## Cheat-sheet (approximate — verify against the live site before shipping)

| Brand | Font → substitute | bg / text / muted | Accent | Signature move |
|---|---|---|---|---|
| **Stripe** | Söhne → Inter | `#ffffff` / `#0a2540` / `#425466` | `#635bff` | Fluid animated gradient hero; soft layered shadows; ~12px radius |
| **Linear** | Inter Display → Inter | `#08090a` / `#f7f8f8` / `#8a8f98` | `#5e6ad2` | Dark-first; `rgba(255,255,255,0.08)` hairlines; gradient glows; tight tracking |
| **Vercel** | Geist / Geist Mono | `#ffffff` / `#171717` / `#666` | `#171717` | `box-shadow:0 0 0 1px rgba(0,0,0,.08)` as border; extreme negative tracking; monochrome |
| **Apple** | SF Pro → Inter/system | `#ffffff`+`#000` / `#1d1d1f` / `#6e6e73` | `#0071e3` | Cinematic full-bleed imagery; vast whitespace; huge display type; ~18px radius |
| **Notion** | Inter + Lyon serif → Inter + Fraunces | `#ffffff` / `#37352f` / `#787774` | `#2383e2` | Warm neutrals; serif headings; small radius; understated |
| **GitHub** | Mona Sans → Inter | `#ffffff`+`#0d1117` / `#1f2328` / `#656d76` | `#0969da` | Dense, functional; `#d0d7de` borders; 6px radius; Primer system |
| **Airbnb** | Cereal → Poppins/Inter | `#ffffff` / `#222222` / `#717171` | `#ff385c` | Coral accent; photography-first; 12px radius; soft shadows; rounded pills |
| **Spotify** | Circular → Montserrat | `#121212` / `#ffffff` / `#b3b3b3` | `#1db954` | Dark canvas; bold type; green accent; pill buttons; album-art driven |
| **Figma** | Inter | `#ffffff` / `#1e1e1e` / `#666` | multi (`#a259ff` `#1abcfe` `#0acf83` `#ff7262`) | Playful multi-color; small radii; canvas-y |
| **Framer** | Inter | `#0a0a0a` / `#ffffff` / `#999` | `#0099ff` | Bold black + electric blue; motion-first; big type |
| **OpenAI** | Söhne / OpenAI Sans → Inter | `#ffffff`+`#000` / `#0d0d0d` / `#6e6e80` | `#000000` | Restrained monochrome; generous whitespace; minimal ornament |
| **Discord** | gg sans → Inter | `#313338`+`#5865f2` marketing / `#ffffff` | `#5865f2` | Blurple accent; rounded 8–16; friendly, chunky |

## Font substitute rules

Proprietary brand fonts can't be loaded — use the closest free CDN face and say so:
Söhne/Circular/Cereal → **Inter**; SF Pro → **Inter** or system stack; Geist → **Geist** (it's free); GT-style grotesks → **Inter**/**Manrope**; serif display → **Fraunces**/**Playfair**. Match the *category and weight range*, not the exact glyphs.

## Fidelity rules

- **Capture the logic, not the surface.** "One restrained accent + shadow-as-border + tight tracking" transfers; the literal `#635bff` is just today's value. Get the system and it survives their next redesign.
- **Adapt to the content.** A mirror of Stripe applied to a bakery isn't Stripe — it's Stripe's *system* wearing the bakery's palette. Keep the structure, swap the surface to fit the actual project.
- **Don't clone assets.** Logos, brand names, copy, illustrations, and photography are the source's property. Mirror the visual *system* for your own content; never lift their trademarked marks or lift their page wholesale. This is inspiration, not reproduction.
- **One signature, not a pixel-diff.** Nail the one recognizable move; don't burn hours matching a gradient stop. Resemblance comes from the system, not from decimal-perfect hexes.

## After extracting

Lock it like any direction (name it "Stripe-like," note the live URL as the reference, pick the adjective triad from its feel), then build the system with the other references. Run `anti-slop.md` at the end — a mirror can still ship flat if you copied colors but skipped hierarchy and polish.
