# Typography (subskill of macos-app-design)

Load when choosing fonts/sizes, or when type feels off-platform. The rule: **use the system font at real macOS text-style sizes**, driven by `preferredFont(forTextStyle:)`, not ad-hoc pixel sizes from web habits.

## The system font

- **SF Pro** is the system font — you get it by asking for the system font, not by bundling anything. `NSFont.systemFont(...)` / SwiftUI `.font(.body)`.
- The system auto-selects the optical variant: **SF Pro Text** below ~20pt, **SF Pro Display** at/above ~20pt (tighter tracking). Don't fight this.
- **SF Mono** for code, terminals, and aligned numeric data: `NSFont.monospacedSystemFont(...)` / `.font(.system(.body, design: .monospaced))`.
- **SF Rounded** (`design: .rounded`) suits friendly/utility apps; use deliberately, not by default.
- A **rounded reading serif** (New York, `design: .serif`) fits content-first writing/reading apps.

Only bundle a custom face for a strong brand display need, and even then keep body/UI text on SF — a bundled UI font is the #1 "this is a ported web app" tell.

## macOS text styles (point sizes)

Use these semantic styles (`NSFont.TextStyle` / SwiftUI `Font.TextStyle`) so type scales with the user's setting. Approximate default macOS sizes:

| Style | ~pt | Weight | Use |
|---|---|---|---|
| `largeTitle` | 26 | regular | screen/hero title (sparingly on Mac) |
| `title` / `title1` | 22 | regular | major section title |
| `title2` | 17 | regular | subsection |
| `title3` | 15 | regular | minor heading |
| `headline` | 13 | **semibold** | emphasized row/label, list item title |
| `body` | **13** | regular | default UI + control text |
| `callout` | 12 | regular | secondary body |
| `subheadline` | 11 | regular | supporting text |
| `footnote` | 10 | regular | captions |
| `caption` / `caption2` | 10 | regular | finest labels |

**13pt is the default control text size on macOS** — much smaller than the web's 16px body. Mac UI is denser; don't inflate everything to web sizes.

## Control sizes

Native controls come in sizes — match them to density (see the archetype):
- `.regular` — standard (13pt).
- `.small` — 11pt, pro/dense panels and inspectors.
- `.mini` — 9–10pt, very tight utility.
- `.large` — prominent primary actions.

SwiftUI: `.controlSize(.small)`. AppKit: `control.controlSize = .small`.

## Weight, tracking, numerals

- Use weight for hierarchy before size: `headline` is body-size but semibold. Keep 2–3 weights (regular, medium/semibold, bold), not a zoo.
- Don't hand-set tracking — SF's optical sizing handles it. Web habits like `letter-spacing: -0.02em` on everything read wrong here.
- **Tabular numerals** for any aligned/columnar numbers (tables, monospaced data, timers): `.monospacedDigit()` / `.fontDescriptor` with the monospaced-digit feature. Non-tabular numbers jitter in lists.

## Dynamic Type & measure

- Prefer text styles over fixed sizes so the app respects the user's text-size setting; test at larger sizes.
- Keep reading measure sensible in content-first apps (~60–75 characters). Don't run body text the full width of a wide window.

## Anti-patterns

- Bundling `Inter`/`Roboto` for UI text — instantly reads as a web port. Use SF.
- Web body sizes (16–18px) across a dense Mac window — everything looks oversized.
- Proportional numerals in a data table — columns wobble; use tabular.
- Hardcoded pixel sizes instead of text styles — breaks Dynamic Type and looks non-native.
