# Title

fix(menus): Open every submenu on hover as Windows 11 does

# Body

Model: claude-opus-5-5

In Windows 11 File Explorer (build 26200), resting the pointer on a menu row with a submenu, such as View > Show, opens the submenu without a click, and resting on another row closes it. OpenXplorer's menus are meant to do the same, but the command bar's submenus often opened only on a click.

The cause was in GTK, not in each menu. The command bar's menus hold no pointer grab of their own, so they can switch from one to another with one click. While such a menu opens, GTK 4.14 on X11 makes up a motion event for the window under it and tells the menu that the pointer left, although it never moved. That canceled the hover delay, so a pointer that came to rest on Show just as View opened never opened Show's submenu. A click still worked.

All of the app's menus share one menu widget, so the fix is one rule in that widget (`menu_popover.rs`), not a change per menu:

- **Hover:** when GTK says the pointer left the rows, the menu now asks the pointer where it is before it gives up the row. Resting on a row for 400 ms (Windows' default menu show delay) opens its submenu beside the menu. Resting on another row for as long closes it. Leaving every menu closes nothing.
- **Moving towards an open submenu:** the pointer heading for the open submenu, inside the triangle between where it was and the submenu's near edge, no longer counts as resting on the rows it crosses, however slowly it moves. Before, crossing another row for more than 400 ms on the way closed the submenu or opened that row's instead.
- **No room beside the menu:** GTK used to close a submenu that fit on neither side of its menu (seen at 200% scale on a small screen), for a click as well as hover. It now opens over the menu, as Windows places it, below its row's end. A submenu taller than half the monitor keeps that place too, with its rows scrolling within the monitor, instead of being fitted back beside the menu where it does not fit.
- Unchanged: a click opens a submenu at once (a tap activates the row the same way, but no touch test was run), Right or Enter opens it with the keyboard in it, and Left or Escape closes it and puts the keyboard back on its row. With animations turned off, submenus show without motion after the same delays. One-click switching between the command bar's menus is unchanged.

The snapshot hook gains a `hover=<label>` scene step, and `tools/native_capture.py` can take a picture at `GDK_SCALE=2` on a screen twice the size. The test pointer now looks for windows across the whole screen, whatever its size.

## Menus with submenus

"Main" is unchanged main, "fixed" is this branch, both driven with a real pointer (XTEST) on Xvfb at 100%.

| Menu | Submenus | Hover on main | Hover fixed |
| --- | --- | --- | --- |
| Command bar Sort | More, Group by | Intermittent: fails when the pointer comes to rest as the menu opens | Yes |
| Command bar View | Show | Intermittent: failed 4 of 4 attempts when resting on Show as View opens | Yes |
| Command bar Search options, Gallery View, See more, New | Date modified, Kind, Size; Show; Disconnect network drive; template folders | Same menu code as Sort and View | Same fix (not driven with the pointer) |
| Right-click, compact, on a file | Open with, Compress to | Yes | Yes |
| Right-click, compact and classic, on blank space | View, Sort by, Group by, New | Yes | Yes |
| Right-click, classic, on a file or folder | Send to | Yes | Yes |
| Right-click Share with, Service actions | their targets | Same menu code (shown only with Bluetooth or mail, or configured actions) | Same fix (not driven with the pointer) |
| Navigation pane, pinned folder | Compress to, New | Yes | Yes |
| Home, Quick access tile | Compress to | Yes | Yes |
| Any menu: slow move towards an open submenu across other rows | | Closes it or opens another | Keeps it open |
| Any menu: submenu with no room on either side | | Closed by GTK, also on click | Opens over the menu |

The tab menu, the column titles' menu, the address bar's menus, Collection and the drop-downs in Settings and dialogs have no submenus.

## How it was tested

- New tests in `native/crates/ox-app/src/window/tests/submenu_hover.rs` drive every submenu above that is shown with the fictional fixture files, using the real pointer and keyboard through XTEST. For each one they check that hover opens it, hover on another row closes it while the menu stays open, a click opens it at once, Right opens it and Left closes it back to its row, and Escape closes only the submenu. They also cover a slow diagonal move, moving away, leaving the menu, the pointer resting on Show as View opens, no room on either side (a short and a tall submenu), and the dark theme with animations off.
- On unchanged main, 4 of the new tests fail on Xvfb at 100%: the pointer resting on Show as View opens (4 of 4 attempts), the slow diagonal move, no room on either side, and the command bar test (View > Show hover). The tall submenu test was added later and fails on the branch before its fix: the submenu went back beside the menu, flipped left, partly off the screen and taller than the monitor. With the fixes, all 9 pass on Xvfb at 100% (two runs in a row) and at 200% on a 2880x2000 screen. On Weston (Wayland, nested in Xvfb) at 100%, the other 7 pass. The two no-room tests can't pass there, because Weston places popups unconstrained: it shows the submenu at full size beside its menu, running off the screen, so GTK never sees it squeezed. Compositors that apply the constraints GTK asks for squeeze it, and GTK then closes it, which is the case the fallback handles; no such compositor was tested here. On Weston at 200%, the test pointer could not reliably find windows and menus, so that combination is not validated.
- `python3 native/parity/check.py` passes. In `python3 native/tools/check.py`, the parity checks, rustfmt and Clippy pass. The test run stopped at the first failing executable, an ox-core test that compares a fixture with this machine's Python `urllib`, so every test executable and the doctests were then run in the same isolated way, one after another: 4,766 tests passed and 55 distinct tests failed. All 55 also fail on unchanged main, run the same way: Python app interop, keyring, administrator copy and snapshot tests in ox-core, and two ox-app thumbnail sandbox tests. None of them involves menus.
- Pictures taken with `tools/native_capture.py` and the fictional demo tree, at 100% and 200%, light and dark, show View > Show and a right-click menu's Compress to opened by the new `hover=` step. In those pictures the snapshot hook draws submenus over or left of their menu; it does the same on unchanged main, so that placement comes from how the hook composes popups into the picture.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
