# Materials & color (subskill of macos-app-design)

Load when choosing surfaces/colors, or when a fill looks hardcoded or breaks in dark mode. The core discipline: **surfaces are materials, colors are semantic.** Almost nothing on a Mac app should be a literal hex.

## Materials (vibrancy)

macOS surfaces are translucent materials that sample what's behind them, not flat fills. Use `NSVisualEffectView` (AppKit) or SwiftUI `.background(.<material>)` / `Material`.

`NSVisualEffectView.Material` — pick by role:

- **`.sidebar`** — sidebars / source lists.
- **`.underWindowBackground`** — the window's base backing surface behind content.
- **`.contentBackground`** — content areas (lists, text).
- **`.headerView`** — section headers / bars.
- **`.titlebar`** — under the title bar.
- **`.menu`, `.popover`, `.hudWindow`** — menus, popovers, HUD panels (menu-bar utilities).
- **`.selection`** — selected-row background (usually let the control do this).

Two blending modes: **`.behindWindow`** (samples the desktop/other windows — sidebars, popovers) vs **`.withinWindow`** (samples the app's own content beneath — inspector over a canvas). SwiftUI: `.regularMaterial`, `.thinMaterial`, `.thickMaterial`, `.ultraThinMaterial`, `.bar` map to these.

Honor **Reduce Transparency** — the material APIs do this for you automatically; hardcoding a translucent hex does not.

## Semantic colors (never literal)

Use system semantic colors so light/dark, Increase Contrast, and the user's accent all adapt with zero branches.

Text / labels (`NSColor` → SwiftUI):
- `labelColor` → `.primary` — primary text.
- `secondaryLabelColor` → `.secondary` — subtitles, captions.
- `tertiaryLabelColor` → placeholder, disabled-ish.
- `quaternaryLabelColor` → faintest (watermarks, dividline hints).

Surfaces & lines:
- `windowBackgroundColor`, `underPageBackgroundColor`, `controlBackgroundColor` — backing surfaces (when not using a material).
- `separatorColor` → `Divider()` — hairline dividers. Never a hardcoded gray line.
- `gridColor` — table grid lines.

Controls & selection:
- `controlAccentColor` → `Color.accentColor` — **the user's chosen accent**. Use it for the primary action, selection, focus — not a brand blue you picked.
- `selectedContentBackgroundColor` — selected list rows.
- `controlColor`, `disabledControlTextColor`.

Status: `systemRed/Green/Orange/...` for semantic status only, not decoration.

In SwiftUI, prefer `.primary`/`.secondary`/`.tint(...)`/`.foregroundStyle(.secondary)` and the `Color(nsColor:)` bridge for the rest.

## Accent color

- Default to the **system accent** (`controlAccentColor` / `.accentColor`) so the app matches the user's System Settings choice.
- If the app has a brand accent, set it once via `.tint(...)` / `NSApp` accent and let it flow everywhere — but keep every other color semantic. One tint, not a repainted palette.
- Selection, focus rings, active toggles, primary buttons = accent. Body text and chrome = never accent.

## Dark mode — for free, if you did it right

If you used materials + semantic colors, dark mode already works. For your *own* custom colors, define them as **asset-catalog colors with Any/Dark appearances** (or `NSColor(name:dynamicProvider:)`), never a single hex. Read the effective appearance with `NSApp.effectiveAppearance` / SwiftUI `@Environment(\.colorScheme)` only when you truly must branch.

Test in both, plus Increase Contrast. If a surface looks wrong in dark, you hardcoded something — find the literal hex.

## Anti-patterns

- A flat `Color(hex: "#F5F5F5")` sidebar instead of the `.sidebar` material.
- `.foregroundColor(.gray)` everywhere instead of `.secondary` — it won't shift for contrast/dark.
- A brand-blue button when `controlAccentColor` exists — you just overrode the user's accent choice.
- A single hardcoded hex for a "card" that inverts wrong in dark mode.
- Translucency faked with opacity instead of a real material (ignores Reduce Transparency).
