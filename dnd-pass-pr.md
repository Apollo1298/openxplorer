# Title

fix(dnd): Copy plain drags into Thunar, take drops on the folder tree and explain drops on an open ZIP

# Body

Model: claude-opus-5-5

This pass tested drag and drop with real mouse input against other apps (Thunar 4.18, Nautilus 46, Nemo, Dolphin, Chromium and a small GTK 4 test app), inside the app (folders, tabs, navigation pane, crumbs, split panes, two windows, ZIP views) and with every modifier. Most cases already worked like Windows. It fixes the four that did not:

- **A plain drag into Thunar asked every time.** The app's plain drag offered Copy and Ask. Thunar opens its "Copy here / Cancel" menu on every drop whose drag offers Ask, so each drag from the app into Thunar (Xfce's file manager) stopped at a menu. A plain drag now offers Copy only, and Thunar copies at once. A right-button drag still offers Ask, so Thunar still shows its menu for it. Shift and Ctrl+Shift still add Move and Link.
- **The folder tree took no drops.** With View > Folder tree on, dropping files on a folder of the tree did nothing. The tree's folders now take drops like the navigation pane's places: the folder under the pointer gets the drop outline and the "Move to Music" label, the drop follows the usual move/copy/ask rules, the tree scrolls near its edge, and a collapsed folder held under a drag for 750 ms opens.
- **A drop on an open ZIP was refused without a word.** Windows adds dropped files to a zipped folder. ZIPs opened like folders are read-only here, and GTK delivers no drop to a place that takes none, so the drag simply failed. The window now says "An archive is read-only. Extract it to add items." once while the drag hovers there. Dragging the archive's own items out says nothing.
- **A green frame surrounded the whole navigation pane and address bar during every drag.** GTK's theme draws it around any widget that holds a drop target. It is gone; only the row, crumb or folder the drop would go into is outlined.

This branch also carries the pending Windows-differences changes (Alt at the drop makes shortcuts) and three other pending branches that touch the same files, merged with conflicts only in documentation text and generated files (the message template and the docs index).

| Case | Before | After | Windows |
|---|---|---|---|
| Plain drag of a file into Thunar | Thunar's "Copy here / Cancel" menu on every drop | Copied at once; right-drag still shows Thunar's menu | NOT CONFIRMED (no Microsoft page found); a plain Explorer drag shows no menu |
| Shift-drag into Thunar | Moved | Moved | "Shift+mouse drag a file: When you drop the file, move it to that location." ([keyboard shortcuts](https://support.microsoft.com/en-us/windows/keyboard-shortcuts-in-windows-dcc61a57-8ff0-cffe-9796-cb9706c75eec)) |
| Alt-drag onto a folder | Copied ("Copy to Archive") | Shortcut made ("Create link in Launch planning") | "Alt+mouse drag a file: When you drop the file, create a shortcut to the original file in that location." (same page) |
| Drop on a folder of the folder tree | Nothing happened | Goes into that folder, outline and "Move to Music" label, collapsed folder opens after 750 ms | NOT CONFIRMED |
| Drop on a ZIP opened like a folder | Silently refused | Refused, with "An archive is read-only. Extract it to add items." | Adds the files: "To add files or folders to a zipped folder you created earlier, drag them to the zipped folder." ([Zip and unzip files](https://support.microsoft.com/en-us/windows/zip-and-unzip-files-8d28fa72-f2f9-712f-67df-f80cf89fd4e5)) |
| Drag over the navigation pane or address bar | Green frame round the whole pane or bar | Only the target row or crumb outlined | NOT CONFIRMED |

Unchanged and working (checked with real input): same-drive move, other-drive copy, Ctrl copy, the right-drag menu (Copy here, **Move here**, Create shortcuts here, Cancel), Escape cancelling a drag, read-only folders ("Destination Folder Access Denied"), a folder dropped into its own subfolder ("The destination folder is a subfolder of the source folder."), the Recycle Bin, crumbs, tab hover (800 ms) and drop on a tab, split panes, drags between windows, 2,999 items at once, drags into Nautilus and Nemo (copied, no menu) and from Thunar into the app (copied).

## CHANGED OWNER DECISIONS

- A plain drag out no longer offers Ask (DND-008 said "A plain drag offers Copy and Ask only"). Only a right-button drag offers it. Holding the desktop's "ask" key (Alt on GNOME) while dragging out to another app therefore no longer opens that app's menu; Alt at a drop inside the app makes shortcuts, as on Windows.
- Included from the pending Windows-differences changes: Alt held at a drop of the app's own items makes shortcuts instead of copying (X11) or asking (GNOME on Wayland).

## NOT CONFIRMED

Check these in Windows 11 File Explorer; no Microsoft page covers them:

- A plain drag from Explorer into another app or the desktop never shows a menu.
- The default move within a drive and copy across drives.
- Explorer draws no frame around the whole navigation pane or address bar during a drag.
- A collapsed folder in the navigation pane opens when a drag rests on it, and after how long; the same for a tab (800 ms here).
- A link dragged from a browser creates a `.url` shortcut, and a picture dragged from a browser is saved as a file. Here, Firefox-style links are refused with "Drop files or folders, rather than links or text.", and Chromium's links and pictures (which carry no file list) are ignored without a message.
- Text dragged onto a folder does nothing.
- Explorer shows "Copy to"/"Move to" labels for drags that come from other apps; here only the app's own drags carry the label.
- The navigation pane's folders can be dragged out; this app's folder tree still cannot.

## How it was tested

- Real input with xdotool on Xvfb (X11) and on Weston 13 (Wayland, its X11 backend), at 1x and 2x, light and dark, with fictional files: every case in the table and the list above. Results were checked on disk; app logs showed no GTK critical warnings and no crashes.
- New tests, each failing before its fix and passing after: `a_plain_drag_offers_copy_without_asking_and_modifiers_add_move_or_link`, `a_real_drag_onto_a_folder_of_the_folder_tree_goes_into_it` (real XTEST drag), `a_drag_onto_an_open_zip_says_it_is_read_only` (real XTEST drag between two windows), `a_drag_over_the_navigation_pane_or_the_address_bar_draws_no_frame_round_it` (pixel check). Also `a_folder_of_the_folder_tree_takes_drops_and_opens_under_a_held_drag` and two capture tests.
- Checks on the final branch: `cargo fmt --check` passes; `python3 native/parity/check.py` passes; the i18n template and the generated docs are current. Clippy (`-D warnings`, the workspace's pedantic lints) reports five errors, all in test code of the Open and Save dialogs merged from another branch (`picker.rs`), none in this change. All 81 test executables and the doctests ran isolated as `native/tools/check.py` runs them, and failures were compared by test name with unchanged main: the same Python-interop, sandboxed-thumbnailer, settings-page and sidebar-width failures fail on both (this machine has Python 3.13 and no FUSE). Failing only on this branch, all from merged branches: two Open and Save dialog layout tests (their labels now carry mnemonics), the Gallery's justified-rows test (thumbnails time out) and the stylesheet scan for raw colours (a `black` in the Gallery's styles).
- Pictures, from the tests' capture hook, at 1x and 2x in both themes: the folder tree's drop outline, the note over an open ZIP, and the navigation pane and address bar under a drag. `tools/audit-public-data.py` reports no issues.
- Limit: at 2x on Weston, Thunar did not take a drop from the app (no drop feedback), while Thunar to the app worked and app to Thunar worked at 1x on Weston and at 1x and 2x on X11. The same 2x case could not be compared with the unchanged app because Weston placed the windows off screen.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
