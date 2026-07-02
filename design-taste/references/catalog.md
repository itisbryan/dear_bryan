# Direction catalog (subskill of design-taste)

Load this at step 1. Two ways to land on a direction: **recommend** one that fits the current project (default), or let the user **choose** from the menu. Every entry is a real, committed visual language with starter tokens — not a brand clone, a direction you then build out with the type/color/space references.

## How to recommend (rubric)

Inspect the project, then map signals → direction. Look at, in order:

1. **Product type** — marketing site, web app/dashboard, docs, portfolio, storefront, landing page.
2. **Audience** — developers · consumers · enterprise/finance · creatives.
3. **Existing brand** — colors, logo, fonts already in the repo (`tailwind.config`, CSS vars, `<link>` fonts, brand assets). If a brand exists, the direction must accommodate it, not fight it.
4. **Stack** — component library in use (shadcn, MUI, Chakra) sets a baseline you'll override; note it.
5. **Current vibe** — if restyling, what it reads as now and what's wrong with it.

Then recommend the row whose **Fit signals** match most, and say why in one line. When two fit, pick the one that matches the *audience* — audience beats aesthetics.

## The menu

| Direction | Feels | Fit signals | References |
|---|---|---|---|
| **Technical / utilitarian** | precise, quiet, engineered | dev tools, infra, APIs; developer audience; data-dense | Linear, Vercel, Stripe |
| **Warm minimal** | calm, human, soft | productivity, wellness, content; consumer; trust via warmth | Notion, Things, Cal.com |
| **Editorial** | authored, confident, considered | publications, agencies, portfolios; storytelling; long-form | Stripe Press, magazines |
| **Luxe** | premium, restrained, expensive | high-end product, finance, luxury brand; status | Apple, high-end fashion |
| **Modern SaaS** | approachable, credible, bright | B2B product marketing, general startup; broad audience | Linear-lite, Framer sites |
| **Corporate / enterprise** | structured, trustworthy, systematic | enterprise, gov, healthcare, fintech; risk-averse buyers | IBM Carbon, Atlassian |
| **Playful** | friendly, energetic, human | consumer apps, education, community; younger audience | Figma, Duolingo, Miro |
| **Brutalist** | raw, deliberate, loud | creative/experimental, portfolios, editorial statements | Gumroad-era, indie zines |

## Starter tokens per direction

Copy the block for the chosen direction, then refine with `typography.md` / `color.md` / `layout.md`.

```css
/* Technical / utilitarian */
--font-sans:'Inter',system-ui,sans-serif; --font-mono:'Geist Mono',ui-monospace,monospace;
--bg:#fafafa; --text:#171717; --text-muted:#6b6b6b; --accent:#0a5fff;
--border:rgba(0,0,0,0.08); --radius:8px; --scale-ratio:1.2; /* dense */

/* Warm minimal */
--font-sans:'Inter','Söhne',system-ui,sans-serif; --font-serif:'Fraunces',Georgia,serif;
--bg:#f8f7f4; --text:#2b2a27; --text-muted:#78756e; --accent:#c2703d;
--border:rgba(0,0,0,0.07); --radius:12px; --scale-ratio:1.25; /* airy */

/* Editorial */
--font-serif:'Freight','Tiempos',Georgia,serif; --font-sans:'Inter',sans-serif;
--bg:#ffffff; --text:#111111; --text-muted:#555; --accent:#1a1a1a;
--border:rgba(0,0,0,0.12); --radius:0px; --scale-ratio:1.333; /* wide measure, asymmetric */

/* Luxe */
--font-serif:'Canela','Playfair Display',serif; --font-sans:'Inter',sans-serif;
--bg:#0d0d0d; --text:#f2f0ea; --text-muted:#9a968c; --accent:#c9a86a;
--border:rgba(255,255,255,0.1); --radius:2px; --scale-ratio:1.414; /* vast whitespace */

/* Modern SaaS */
--font-sans:'Geist','Inter',system-ui,sans-serif;
--bg:#ffffff; --text:#0f172a; --text-muted:#64748b; --accent:#6366f1;
--border:rgba(15,23,42,0.08); --radius:10px; --scale-ratio:1.25;
/* NB: this is the slop-adjacent one — earn it with real hierarchy, tasteful (not default) accent, layered shadows */

/* Corporate / enterprise */
--font-sans:'IBM Plex Sans','Inter',sans-serif; --font-mono:'IBM Plex Mono',monospace;
--bg:#ffffff; --text:#161616; --text-muted:#525252; --accent:#0f62fe;
--border:#e0e0e0; --radius:4px; --scale-ratio:1.2; /* structured grid, low ornament */

/* Playful */
--font-sans:'Poppins','Nunito',system-ui,sans-serif;
--bg:#fffdf7; --text:#1f2933; --text-muted:#5c6773; --accent:#ff5c8a; --accent-2:#4ecdc4;
--border:rgba(0,0,0,0.06); --radius:16px; --scale-ratio:1.25; /* bold weights, bouncy motion */

/* Brutalist */
--font-mono:'Space Mono',monospace; --font-sans:'Helvetica Neue',Arial,sans-serif;
--bg:#ffffff; --text:#000000; --accent:#0000ff;
--border:2px solid #000; --radius:0px; --scale-ratio:1.5; /* stark, exposed grid, huge type */
```

## After the pick

- If a **brand already exists** in the project, override the starter `--accent` and fonts with the real brand tokens — keep the direction's *structure* (scale, spacing, shadow philosophy), swap its surface colors.
- Lock the three artifacts (named direction · 2–3 references · adjective triad) per `direction.md`, then build the system.

## Anti-patterns

- Defaulting to **Modern SaaS** for everything because it's safe — that's how the indigo-gradient slop happens. Recommend it only when the fit signals actually point there.
- Offering the whole menu with no recommendation when you *can* read the project — do the work, then let them override.
- Cloning a reference's exact palette instead of adopting the direction's logic. Steal the system, not the hex.
