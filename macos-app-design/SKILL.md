---
name: macos-app-design
description: Guides distinctive, native-feeling macOS desktop app GUI design — recommends or lets you pick a Mac app archetype (system-native, pro/dense, content-first, menu-bar utility, prosumer-creative), then builds a real system of window chrome, materials, semantic colors, SF type, and native controls so the app reads as a real Mac app, not a web page in a window. Loads focused references (catalog, hig-structure, materials-color, typography, controls, motion-polish, anti-slop) on demand. Use when designing or restyling a macOS app UI in SwiftUI or AppKit, or when a Mac app "feels like Electron / like a website / not native."
---

# macOS App Design

Helps you design macOS desktop GUIs that read as **native, not ported**. This skill is about the taste and the process — inferring the right Mac app archetype for a brief, committing to real platform materials and chrome, and refusing the tells that make an app feel like a web page stuffed into a window. It supplies judgment and concrete platform values, not a UI kit.

## The one rule that overrides everything

**Feel native. Respect the platform; never port a web app into a window.**

A real Mac app is built from the system, not around it:
- **System materials, not flat fills** — sidebars and title bars use vibrancy (`NSVisualEffectView` / SwiftUI materials), not a hardcoded gray hex.
- **Semantic colors, not literal ones** — `labelColor`, `secondaryLabelColor`, `controlAccentColor`, `separatorColor` — so light/dark and the user's accent color Just Work. A `#333` label is a bug waiting for dark mode.
- **SF, not a bundled web font** — the system font at real macOS text-style sizes, not `Inter 16px`.
- **Native controls + standard chrome** — toolbar, sidebar, split view, real buttons, disclosure triangles. Plus the menu bar and keyboard shortcuts, which web ports forget.
- **Adapts automatically** — light mode, dark mode, Increase Contrast, Reduce Motion, Reduce Transparency, Dynamic Type, and the accent color all handled without a code branch.

If a surface is a literal hex, a bundled web font, a `div`-shaped custom control, or a window with no menu bar and no keyboard support — you haven't designed a Mac app, you've skipped the platform. See [`references/anti-slop.md`](./references/anti-slop.md) for the specific tells and the pre-flight check.

## The process

### 0. Audit first (if something already exists)

Restyling, not greenfield? Look at the current app before touching it. Name what already feels native (keep it), what reads as web-ported or off-platform (the target), and the constraints you can't break (AppKit vs SwiftUI, minimum OS version, existing brand accent). Don't re-native what's already fine.

### 1. Pick an archetype — recommend, or let them choose

Every Mac app commits to **one** archetype. The archetype decides how much chrome, how dense, how much custom canvas. Get there by:

- **Recommend (default):** inspect the app — what it does, who uses it, how information-dense it is, whether it's a full-window app or a menu-bar utility — and recommend the archetype that fits, with the reasoning. Use the rubric in [`references/catalog.md`](./references/catalog.md). Say *why* ("developer tool, panel-heavy, dark-first → Pro/dense, like Xcode").
- **Choose:** present the archetype menu and let the user pick. Honor an explicit pick, but flag a genuine mismatch once ("A full toolbar + sidebar on a single-purpose menu-bar timer will feel heavy — sure?").

Lock two artifacts before building: the named archetype, and 2–3 real Apple or best-in-class Mac apps as references (Finder, Notes, Xcode, Things, Reminders…). The deeper guidance is in [`references/catalog.md`](./references/catalog.md). This is the highest-leverage step and the most-skipped — get the archetype wrong and every later choice fights the app.

### 2. Build the structure before the pixels

macOS design is structure-first: the window shape decides everything downstream. Settle the chrome — window style, toolbar, sidebar/split view, inspector, content region, and the layout metrics — *before* styling any single control. Load [`references/hig-structure.md`](./references/hig-structure.md).

### 3. Establish materials, color, and type

The archetype implies a token set. Set them before laying out components:
- Materials, semantic colors, accent, dark mode → [`references/materials-color.md`](./references/materials-color.md)
- Type (SF, text styles, control sizes) → [`references/typography.md`](./references/typography.md)
- Native controls vocabulary, SF Symbols, menu bar & keyboard → [`references/controls.md`](./references/controls.md)

### 4. Compose with hierarchy and native rhythm

Lay out from the metrics — standard window margins, the 8/20 spacing rhythm, aligned control baselines. One region is clearly primary; the sidebar and inspector are subordinate. Squint test: at a glance, is it obvious this is a Mac app and where the focus is?

### 5. Polish and motion

The gap between "works" and "feels Apple-made" is micro-detail: focus rings, hover reveals, disclosure animations, restrained spring motion, and honoring Reduce Motion / Reduce Transparency. Load [`references/motion-polish.md`](./references/motion-polish.md).

### 6. Pre-flight before you ship

Run the checklist in [`references/anti-slop.md`](./references/anti-slop.md). If any answer is "literal / hardcoded / no menu bar," you're not done.

## References (load on demand)

Load only on the trigger, not speculatively — progressive disclosure is the point. Each file carries concrete platform values (real materials, semantic color names, point sizes, symbol configs), the same spirit as this skill's one rule.

| Trigger | Load | What it does |
|---|---|---|
| Step 1 — pick a Mac app archetype or recommend one | [`references/catalog.md`](./references/catalog.md) | The archetype menu (system-native, pro/dense, content-first, menu-bar utility, prosumer-creative) + a rubric to recommend the fit, with reference apps. |
| Step 2 — deciding window chrome, sidebar, toolbar, split view, layout metrics | [`references/hig-structure.md`](./references/hig-structure.md) | Window styles & toolbar modes, sidebar/split view, inspector, full-size content, standard margins and the 8/20 spacing rhythm. |
| Choosing surfaces/colors, or a fill looks hardcoded / breaks in dark mode | [`references/materials-color.md`](./references/materials-color.md) | Vibrancy materials, semantic `NSColor`/SwiftUI colors, accent color, real dark mode, Reduce Transparency. |
| Choosing fonts/sizes, or type feels off-platform | [`references/typography.md`](./references/typography.md) | SF Pro Text/Display + SF Mono, macOS text styles with point sizes, control sizes, Dynamic Type. |
| Choosing controls, icons, menus, or reaching for a custom widget | [`references/controls.md`](./references/controls.md) | The native control vocabulary + when to use each, SF Symbols rendering, the menu bar and keyboard shortcuts. |
| Something feels static or cheap despite being correct | [`references/motion-polish.md`](./references/motion-polish.md) | Focus rings, hover reveals, disclosure/spring motion, window animation, Reduce Motion, optical alignment. |
| Before shipping, or the app "feels like Electron / a website" | [`references/anti-slop.md`](./references/anti-slop.md) | The non-native tells to kill + the pre-flight checklist. |

Rules for loading references:
- Load only when its trigger fires. Follow that file's guidance fully once loaded.
- A reference is part of this skill, not a separate skill — don't announce a skill switch; just use it.

## Tone

- Opinionated but not precious. Recommend an archetype; don't survey five.
- Concrete and platform-true. "Use `.underWindowBackground` material for the sidebar" beats "make the sidebar translucent."
- Show the reasoning so the taste transfers: name *why* a choice is the native one.

## When NOT to use this skill

- The user wants raw feature/logic code with no visual surface → normal coding workflow.
- Designing a **web** UI, not a Mac app → use the `design-taste` skill instead.
- A design system / brand tokens are already mandated and fixed → apply them, don't re-derive.
- The user wants to *learn* macOS UI development rather than get output → use the `tutor` skill.
- Deep accessibility audit beyond native adaptivity, or App Store review specifics → out of scope; hand off.
