# Window structure & layout (subskill of macos-app-design)

Load this at step 2. The window shape decides everything downstream, so settle chrome and metrics before styling a single control. All values are macOS points (1pt = 1px @1x).

## Window styles

- **Standard titled window** — title bar + traffic lights. The default; use unless you have a reason not to.
- **Unified toolbar** — toolbar merges into the title bar (the modern Mac look). SwiftUI: `.toolbar` on a `NavigationSplitView` gives this for free. AppKit: `window.toolbarStyle = .unified` (or `.unifiedCompact` for denser pro apps).
- **Full-size content view** — content extends under the title bar for a chromeless/immersive look (content-first apps). AppKit: `.fullSizeContentView` + `titlebarAppearsTransparent = true`. Keep traffic lights clear of your content.
- **Panel / popover** — `NSPanel` or SwiftUI popover for menu-bar utilities and inspectors; lighter chrome, often non-activating.

Never draw your own traffic lights or fake title bar. Use the real one and inset content from it.

## Toolbar

- Lives in the title bar (unified). Items are `NSToolbarItem` / SwiftUI `ToolbarItem`, use SF Symbols, and respect the user's toolbar customization + icon/text display mode.
- Primary actions left, search/utility right. Put the sidebar toggle at the far left.
- Toolbar styles: `.unified` (standard), `.unifiedCompact` (pro/dense), `.expanded`, `.preference` (for a Settings window's tab strip).
- Don't cram a web-style nav bar of text links into the toolbar — it's for a few icon actions, not navigation chrome.

## Sidebar & split view

The spine of most Mac apps. Use `NavigationSplitView` (SwiftUI) or `NSSplitViewController` (AppKit):

- **Two-column:** sidebar | detail (Notes-lite).
- **Three-column:** sidebar | list | detail (Mail, Notes). The classic system-native layout.
- Sidebar uses the **`.sidebar` material** (translucent, behind-window vibrancy) and **source-list styling** — SF Symbol + label rows, section headers, disclosure groups. SwiftUI: `List { … }.listStyle(.sidebar)`.
- Metrics: sidebar min ~180–200pt, ideal ~220–260pt; make it collapsible. Selection uses `selectedContentBackgroundColor` (a rounded accent-tinted pill in modern style), never a hard blue rectangle you drew.

## Inspector (right panel)

Pro and creative apps carry a right inspector. SwiftUI 15+: `.inspector(isPresented:)`. Otherwise a right split-view column. Collapsible, ~260–320pt, grouped labeled controls, right-aligned labels or `Form`/`LabeledContent`. Keep it a peer of the content, not modal.

## Content region & layout metrics

The classic Aqua rhythm — internalize these:

- **Window edge margin:** 20pt from content to window/pane edge (`.padding(20)` at the top level, roughly).
- **Between related controls:** 8pt.
- **Between control groups / sections:** 20pt.
- **Label ↔ its control:** 8pt; align labels on a common right edge in forms.
- **Between an icon and its label:** ~6–8pt.
- **Row height in lists:** ~24–28pt comfortable, ~20–22pt dense.

Use layout guides, not magic numbers scattered everywhere: SwiftUI `Form`/`Grid`/`LabeledContent` and AppKit `layoutMarginsGuide` / `NSStackView` give you the standard metrics automatically. Reach for `Settings`/`Form` for preference-style screens.

## Settings window

macOS apps put preferences in a dedicated **Settings** window (⌘,), not an in-app modal. SwiftUI: the `Settings { }` scene with `TabView { }.tabViewStyle(...)` for the toolbar-tab look. Keep it small, fixed-ish width, one pane per tab.

## Safe areas & full screen

Respect the title-bar safe area when using full-size content view. Support full-screen and window resizing — set sensible `minSize`; let split panes and content reflow. Don't lock the window to one size unless it's a fixed utility panel.

## Anti-patterns

- A single giant scroll view with a web-style top nav bar and no sidebar/toolbar — that's a web page, not a Mac window.
- Hardcoded content insets instead of the 8/20 rhythm — spacing reads arbitrary.
- A non-resizable main window, or content that doesn't reflow on resize.
- Faking the title bar / traffic lights instead of using `fullSizeContentView` with the real ones.
