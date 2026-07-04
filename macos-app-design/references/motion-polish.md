# Motion & polish (subskill of macos-app-design)

Load when the app feels static or cheap despite being correct. The gap between "works" and "feels Apple-made" is micro-detail and restrained motion — and honoring the accessibility switches that a web port ignores.

## Focus & the four states

- **Focus ring** — the focused control shows the system focus ring (accent-tinted). Let the system draw it; never suppress it for looks. Keyboard users depend on it. Custom controls must draw an equivalent.
- **Hover** — on Mac, hover *reveals*: row action buttons, disclosure affordances, and scrollbars appear on hover, not permanently. Toolbar/list items get a subtle highlight. Use `.onHover` / tracking areas; keep it quiet.
- **Pressed** — controls dim/tint on mouse-down; native controls do this. 
- **Disabled** — reduced-contrast, non-interactive (`disabledControlTextColor` / `.disabled(true)`), not just greyed with a hex.

## Selection & lists

- Selected rows use `selectedContentBackgroundColor` — in modern style a rounded accent pill inset from the edges, not a full-bleed hard rectangle you drew.
- Inactive-window selection dims to gray automatically with semantic colors — another reason not to hardcode the blue.

## Motion — restrained and purposeful

macOS motion is subtle and functional, never bouncy-for-fun:
- **Durations:** ~0.2–0.35s for most transitions. Disclosure triangles, sidebar collapse, inspector show/hide, popover present.
- **Curves:** ease-in-out or a gentle spring. SwiftUI `.animation(.smooth, ...)` / `.spring(response: 0.3, dampingFraction: 0.9)`; AppKit `NSAnimationContext` / implicit layer animations.
- Animate **state**, not decoration: an expanding section, a value updating, a panel sliding in. No looping ambient motion, no gratuitous parallax.
- Window/sheet/popover present-dismiss uses the system animations — don't replace them.

## Reduce Motion & Reduce Transparency — required

- **Reduce Motion:** check `NSWorkspace.shared.accessibilityDisplayShouldReduceMotion` / SwiftUI `@Environment(\.accessibilityReduceMotion)` and swap animations for instant/cross-fade transitions.
- **Reduce Transparency:** the material APIs fall back to solid automatically — but if you faked vibrancy with opacity, it won't. Another reason to use real materials (`materials-color.md`).
- **Increase Contrast:** semantic colors adapt; verify custom colors do too.

## Optical alignment & micro-detail

- Align on optical centers, not just bounding boxes — an SF Symbol next to a label often needs a ~1pt nudge; use `.firstTextBaseline` alignment in stacks/`LabeledContent`.
- Hairline separators = `separatorColor` / `Divider()`, 1px, not a heavy gray bar.
- Consistent corner radii within a surface; let native controls keep their system radius rather than reshaping them.
- Right-align form labels on a common edge; align numeric columns on the decimal with tabular figures.
- Give the window sensible `minSize` and let content reflow — resize behavior *is* polish.

## Anti-patterns

- Suppressing the focus ring "because it looks cleaner" — breaks keyboard use.
- Permanent row action buttons and always-visible scrollbars (web habit) instead of hover-reveal.
- Bouncy/long/looping animation — reads as a toy, not a tool.
- Ignoring Reduce Motion/Transparency — the app fights the user's accessibility settings.
- A hard full-width blue selection bar instead of the inset accent pill.
