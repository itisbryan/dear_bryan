# Archetype catalog (subskill of macos-app-design)

Load this at step 1. Two ways to land on an archetype: **recommend** the one that fits the app (default), or let the user **choose** from the menu. An archetype is a committed structural stance — how much chrome, how dense, how much custom canvas — not a color scheme. It decides window shape and control density before any pixel.

## How to recommend (rubric)

Inspect the app, then map signals → archetype. Look at, in order:

1. **Job** — single-purpose utility · multi-pane document/library app · creative canvas · dashboard/monitor · settings-heavy.
2. **Surface** — full-window app, or a menu-bar extra / popover?
3. **Density** — a few controls, or panels + inspectors + lists packed with data?
4. **Audience** — everyday consumer · pro/creative · developer · enterprise.
5. **Frame in play** — SwiftUI vs AppKit, minimum OS, existing brand accent to honor.

Then recommend the row whose **Fit signals** match most, and say why in one line. When two fit, pick by *surface first* (menu-bar utility is its own world), then *density*.

## The menu

| Archetype | Feels | Fit signals | Reference apps |
|---|---|---|---|
| **System-native (Aqua)** | familiar, invisible, trustworthy | consumer/productivity, sidebar + list + detail, "should feel like it shipped with the OS" | Finder, Notes, Mail, Reminders |
| **Pro / dense** | powerful, panel-driven, dark-first | pro tools, developer/creative, inspectors + toolbars + info density | Xcode, Logic Pro, Final Cut |
| **Content-first / focused** | calm, chromeless, typographic | writing, reading, single-document focus; hide chrome, whitespace-forward | iA Writer, Ulysses, Things |
| **Menu-bar utility** | compact, glanceable, instant | single job, lives in the menu bar, popover or small panel, no Dock presence | Bartender, Dato, Itsycal, CleanShot |
| **Prosumer creative** | custom canvas in native chrome | design/media tools; big custom work area framed by native toolbar + inspector | Sketch, Pixelmator Pro, Figma desktop |

## Structural starter per archetype

Copy the stance for the chosen archetype, then build it out with `hig-structure.md` / `materials-color.md`.

```
System-native (Aqua)
  Window: standard titled + unified toolbar; NavigationSplitView (sidebar | list | detail)
  Sidebar: .sidebar material, source-list style, SF Symbols; ~200pt min
  Density: comfortable; standard 20pt margins, 8pt control rhythm
  Color: mostly semantic; accent = system accent (respect user's choice)

Pro / dense
  Window: unified toolbar (.unifiedCompact); left source list + center canvas + right inspector
  Panels: .contentBackground / .underWindowBackground; collapsible inspector
  Density: high; .small (11pt) controls, tight rows, tabular numerals
  Color: dark-first, semantic; accent used sparingly for selection/active only

Content-first / focused
  Window: full-size content view, toolbar hidden or auto-revealed; minimal or no sidebar
  Density: airy; wide margins, long measure, one column
  Type: text style hero, high line-height; SF or a real reading face
  Color: near-paper light + true dark; almost no accent

Menu-bar utility
  Surface: NSStatusItem + popover (.popover material) or small NSPanel
  Density: compact; .small/.mini controls, ~280–360pt wide popover
  Chrome: no toolbar; a gear/… menu; Quit + Preferences reachable
  Color: semantic; vibrant popover material behind content

Prosumer creative
  Window: unified toolbar; thin left tools + big custom canvas + right inspector
  Canvas: your custom drawing; everything *around* it is native
  Density: medium chrome, dense inspector; .small controls in panels
  Color: neutral canvas surround (semantic), content is the color
```

## After the pick

- If a **brand accent already exists**, wire it as the accent/tint but keep everything else semantic — never replace `labelColor`/materials with brand hexes. Steal one accent, keep the platform.
- Lock the two artifacts (named archetype · 2–3 reference apps), then settle the window structure in `hig-structure.md`.

## Anti-patterns

- Defaulting to **Pro/dense** because it looks "serious" — density you don't need reads as cluttered. Match the job.
- Building a full **System-native** three-pane window for a one-job utility that wants to be a **menu-bar** popover.
- Copying an Apple app's exact layout without its *reason* — adopt the archetype's structure, not a screenshot.
