# Anti-slop (subskill of macos-app-design)

Load before shipping, or when a Mac app "feels like Electron / a website / not native." This is the enforcement arm of the one rule — *feel native; never port a web app into a window.* It names the tells and gives the pre-flight gate.

## The tells (recognizable non-native / web-port defaults)

If you spot these, the platform was skipped, not designed:

- **No menu bar (or a stub one)** — only "App > Quit," actions unreachable by keyboard. The single loudest tell.
- **Bundled web UI font** — `Inter`/`Roboto`/`Helvetica` for UI text instead of SF. (`typography.md`)
- **Web body sizes** — 16px everything in a window where 13pt is the native control size; the app looks oversized and sparse.
- **Hardcoded hex surfaces** — a `#F5F5F5` sidebar / `#333` label instead of the `.sidebar` material and `labelColor`. Breaks in dark mode. (`materials-color.md`)
- **Flat fills where vibrancy belongs** — opaque sidebar/title bar with no material; ignores Reduce Transparency.
- **Brand-blue everywhere** — a picked blue instead of `controlAccentColor`; overrides the user's accent choice.
- **Custom `div`-style controls** — hand-rolled toggles, dropdowns, checkboxes that lose keyboard, focus, and accessibility. (`controls.md`)
- **Emoji or a web icon font** — instead of SF Symbols.
- **A web top-nav bar** — horizontal text-link navigation across the top instead of a sidebar + toolbar. (`hig-structure.md`)
- **Suppressed focus ring** — "cleaner" look, broken keyboard navigation.
- **Always-on hover affordances** — permanent row buttons and scrollbars instead of hover-reveal. (`motion-polish.md`)
- **Bouncy / looping animation** — reads as a toy, not a Mac tool.
- **Non-resizable window / content that doesn't reflow** — no `minSize`, fixed layout.
- **No dark mode, or it inverts wrong** — a giveaway that colors are literal, not semantic.
- **Proportional numerals in tables** — columns jitter; needs tabular figures.

## Pre-flight checklist

Answer each. If any answer is "literal / hardcoded / no / not sure," you're not done — go back to that reference.

- [ ] **Archetype** — can you name the one archetype and 2–3 reference Mac apps? (`catalog.md`)
- [ ] **Structure** — real window chrome (toolbar/sidebar/split view as the archetype needs), 8/20 spacing rhythm, resizable with `minSize`? (`hig-structure.md`)
- [ ] **Materials** — sidebars/panels use real vibrancy materials, not flat hex fills? (`materials-color.md`)
- [ ] **Color** — every surface/label a semantic color; accent = `controlAccentColor` unless one brand tint; passes in light **and** dark + Increase Contrast? (`materials-color.md`)
- [ ] **Type** — SF (not a bundled web font), macOS text styles at native point sizes, tabular numerals for data? (`typography.md`)
- [ ] **Controls** — native controls (no custom `div` widgets), SF Symbols, and a real menu bar with keyboard shortcuts? (`controls.md`)
- [ ] **Polish** — focus rings visible, hover-reveal, restrained motion, Reduce Motion + Reduce Transparency honored? (`motion-polish.md`)
- [ ] **Native intent** — could you defend every surface as a platform decision, not a web leftover?

## The glance test

Screenshot the app and drop it next to a real Apple app (Finder, Notes, Reminders). Does yours read as a sibling — same chrome language, same material depth, same type weight — or does it read as a website someone put a window frame around? If the second, you skipped the platform; trace it to the failed checklist row.

## When you find slop

Don't patch one symptom. Trace it to the skipped step: hardcoded colors → no semantic/material system (`materials-color.md`); oversized flat UI → web type + no vibrancy (`typography.md` + materials); custom broken widgets → didn't use native controls (`controls.md`); no keyboard → no menu bar (`controls.md`). Fix the system and the tells across the whole app resolve at once.
