# Native controls, icons & menus (subskill of macos-app-design)

Load when choosing controls or icons, or when tempted to build a custom widget. The rule: **use the native control; a custom one must earn it.** A hand-rolled toggle is worse in every way (keyboard, accessibility, dark mode, focus) than the system one — and it's the loudest "web app in a window" tell.

## The control vocabulary — when to use each

| Need | Native control (AppKit / SwiftUI) | Notes |
|---|---|---|
| Trigger an action | Push `Button` | Default action gets ⏎; use `.borderedProminent` for the one primary. |
| On/off | `NSSwitch` / `Toggle` | Switch for settings; `checkbox` style Toggle in dense forms. |
| One of a few | `NSSegmentedControl` / `Picker(.segmented)` | ≤ ~5 options, all visible. |
| One of many | pop-up button / `Picker(.menu)` | Many options, collapsed. |
| Multi-select from short list | checkboxes | Independent booleans. |
| Mutually exclusive short list | radio group | Rare on modern Mac; prefer segmented/pop-up. |
| Number/range | `NSSlider`/`Stepper`, or a stepper + field | Steppers for precise small ranges. |
| Text | `NSTextField` / `TextField`, `SecureField`, `NSTextView` for rich | Give fields a clear label. |
| Show/hide detail | `DisclosureGroup` / disclosure triangle | The native expand affordance. |
| Tabular data | `NSTableView` / `Table` | Sortable columns, tabular numerals. |
| Hierarchy | `NSOutlineView` / `List` with children | Source lists, trees. |
| List / source list | `List(.sidebar)` | Sidebar navigation. |
| Search | `NSSearchField` / `.searchable` | Toolbar-right or list header. |
| Contextual actions | context menu / `.contextMenu` | Right-click; mirror in the menu bar. |
| Transient panel | `NSPopover` / `.popover` | Menu-bar utilities, inline detail. |

Prefer `Form`, `LabeledContent`, `GroupBox`, and `Table` — they carry the standard metrics and alignment for free.

## SF Symbols

- Icons come from **SF Symbols** — a huge system set that matches SF's weight and baseline. Don't ship emoji or a random web icon font as UI icons.
- Weight/scale should match adjacent text: `NSImage.SymbolConfiguration(pointSize:weight:)` / SwiftUI `Image(systemName:).imageScale(...).fontWeight(...)`. A toolbar symbol next to 13pt text should read at that weight.
- Rendering modes: **monochrome** (default, tints with accent), **hierarchical** (one color, layered opacity), **palette** (2–3 explicit colors), **multicolor** (symbol's own colors). Use hierarchical for depth without picking colors.
- Pick semantically correct symbols (`trash`, `square.and.arrow.up`, `gearshape`) — users know them. Check availability against your minimum OS.

## Menu bar — not optional

A real Mac app has a real menu bar. Web ports forget it; users notice immediately.
- Standard menus: **App** (About, Settings…, Services, Quit), **File**, **Edit** (with working Undo/Redo, Cut/Copy/Paste), **View**, **Window**, **Help**.
- Every meaningful action should have a menu item, and important ones a **keyboard shortcut**. SwiftUI: `.commands { CommandMenu(...) { Button(...).keyboardShortcut(...) } }`.
- Wire the standard shortcuts: ⌘, (Settings), ⌘W (close), ⌘N (new), ⌘F (find), ⌘Z/⇧⌘Z (undo/redo), ⌘Q (quit). Don't reinvent them.

## Keyboard & focus

- Full keyboard navigation: Tab moves focus, ⏎ triggers the default button, Esc cancels. Native controls give you this; custom ones usually don't.
- Show a visible **focus ring** on the focused control (the system draws it — don't suppress it). See `motion-polish.md`.

## Anti-patterns

- A custom `div`-style toggle/dropdown instead of `Toggle`/`Picker` — loses keyboard, accessibility, dark mode, focus.
- Emoji or a web icon font where SF Symbols belong.
- No menu bar, or a menu bar with only "App > Quit" — actions unreachable by keyboard.
- Buttons with no default-action (⏎) or no shortcuts — feels like a website, not an app.
- Reinventing right-click behavior instead of `.contextMenu`.
