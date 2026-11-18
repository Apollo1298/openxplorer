# Title

fix(ui): Make drive sizes, key names, tiny pictures, settings search, screen-reader names and split view columns match what the window shows

# Body

Model: claude-opus-5-5

## Problem and resulting behavior

Several small things disagreed with what Windows shows, or with the app itself:

- **Drive sizes.** This PC rounded a drive's sizes while the drive's Properties truncated them as Explorer does, so one drive had two capacities. Both now read the space through one function and write it with one formatter.
- **"1 bytes".** Sizes spelled out in bytes now pick their form through ngettext, so the File Conflict window says "1 byte".
- **Keyboard shortcuts window.** It used GTK's spelling ("Shift+Ctrl+N", "Shift+Alt+P", "Escape", "Space", "Left"). It now writes keys as Microsoft's list and the app's menus do: "Ctrl+Shift+N", "Alt+Shift+P", "Esc", "Spacebar", "Alt+Left arrow". It also lists the rest of Microsoft's File Explorer keys that work: Ctrl+arrow keys then Ctrl+Spacebar, Page Up, Page Down, Enter, Shift+right-click, and Ctrl, Shift or Alt with a drag. Source: https://support.microsoft.com/en-us/accessibility/windows/keyboard-shortcuts-in-windows ("File Explorer keyboard shortcuts").
- **New Bitmap image.** The new picture is 1x1 white, so its thumbnail made the row look iconless in the light theme. A picture too small to see as a thumbnail now keeps its type icon.
- **Settings search.** "hidden files" also found How sizes are counted, and "language" found nothing. A Language row on Appearance now names the language the window is in, or says it is in English because no translation for the desktop's language is installed. Rows with nothing to change read as help: Calculate folder sizes joined How sizes are counted, and the Dragging tabs and files rows show their text under their titles. That help now says Shift moves, Ctrl copies and Alt creates shortcuts, as a drop does, where it said "Alt to choose".
- **Screen-reader names.** The Details button had no name. The search box was named "Search filenames and paths" while it shows "Search Documents". Every tab was described as "Close tab", and the row of column titles repeated its hint once per column. Rows were named with the extension the list hides. Worst, the column titles were paired with their columns in build order, not shown order, so the Date modified title was announced as "Date deleted", and Enter on it sorted by Date deleted. All of these now match what the window shows.
- **Split view at 1366 pixels.** With the details pane open, each pane cut Type off mid-word and hid Size. The only way to reach them was a scroll bar that shows only under the pointer. A horizontal scroll bar now stays shown over the list's bottom edge while the columns are wider than the pane. The bar never changes the list's height, so a pane scrolled while hidden keeps its place.
- **Simplified Chinese.** The catalog is merged with the current template, and the messages shown in English are translated, in two commits of their own that can be dropped together. Eighteen photo and audio property values whose Windows wording is not known with confidence stay in English.

| | Before | After |
|---|---|---|
| Local Disk on This PC vs. its Properties | 27.6 GB free of 252 GB vs. 27.5 GB of 251 GB | 27.5 GB free of 251 GB in both |
| File Conflict window, a 1-byte file | 1 bytes | 1 byte |
| Keyboard shortcuts, New folder / Details pane / Properties | Shift+Ctrl+N, Shift+Alt+P, Alt+Return (and Escape) | Ctrl+Shift+N, Alt+Shift+P, Alt+Enter (and Esc) |
| Keyboard shortcuts, keys from Microsoft's list | Page Up/Down, Enter, Shift+right-click and drag keys missing | All listed |
| New Bitmap image in Details, light theme | No visible icon | Picture type icon |
| Settings search "hidden files" / "language" | Also How sizes are counted / nothing | Show hidden files only / Language |
| Settings rows with nothing to change | Title and ⓘ only | Help text under the title |
| Screen reader: Details button, search box, tab | (empty), "Search filenames and paths", "Close tab" | "Details", "Search Documents", the tab's location |
| Screen reader: Date modified title | "Date deleted", Enter sorted by Date deleted | "Date modified", Enter sorts by Date modified |
| Screen reader: a row | "Budget 2026.txt" | "Budget 2026", as shown |
| Split view at 1366 px with the details pane | Type cut off, Size hidden, no scroll bar | Scroll bar shown; every column reachable |
| Simplified Chinese | About 590 messages in English | 18 in English |

## Validation

- **Tests first.** Each fix has a test that fails on unchanged main and passes here:
  - `format.rs` one_byte_is_singular
  - `landing.rs` a_drive_tile_and_its_properties_write_the_same_sizes
  - `shortcuts_window.rs` the_shortcuts_window_spells_keys_as_windows_does and the_shortcuts_window_lists_every_key_of_microsofts_file_explorer_list
  - `view_styles.rs` a_tiny_picture_keeps_its_type_icon
  - `settings_page/tests.rs` the_search_finds_settings_by_common_words and every_settings_row_has_a_control_or_a_status
  - `accessibility.rs` screen_readers_hear_what_the_window_shows (on main: Details button unnamed, tab without its own description, title row unnamed, Enter on Date modified sorting by another column)
  - `narrow_windows.rs` a_split_tab_beside_the_details_pane_at_1366_pixels_reaches_every_column
- **Checks run.**
  - `python3 native/parity/check.py`: passed.
  - `python3 native/tools/i18n.py check`: passed.
  - `cargo fmt --check` and Clippy with the workspace lints: passed.
  - `python3 native/tools/check.py`: the inventory, icon, format and lint steps passed. It stops at the first failing test executable, so every test executable and the doctests were then run one by one with check.py's own isolation functions.
  - The doctests passed. Thirteen tests fail in this container on unchanged main too, the same by name: nine `admin_copy` tests, two sandboxed-thumbnailer tests, `python_fixture_matches_desktop_core_py` and `fuse_export_paths_match_native_opening_py`. Two more failed once each in a full run and passed every time they were run on their own: `search_history::a_tab_on_results_keeps_its_selection_and_scroll_position` (2 of 2) and `launch::a_folder_named_like_shell_code_stays_a_name` (3 of 3). Everything else passed.
- **Real input, X11.**
  - xdotool key presses on a private Xvfb display with openbox, its own private D-Bus bus and a fictional home. What the app showed was read through AT-SPI.
  - 23 checks in each of 8 combinations (1x and 2x, light and dark, normal and Compact view): all passed.
  - Covered: Ctrl+?, Esc, New > Bitmap image from the keyboard, Settings search, This PC, a paste conflict up to the File Conflict window, and View > Split view with the details pane at 1366 px.
  - No crash and no GTK criticals.
- **Real input, Wayland.**
  - The same 23 checks, with xdotool keys sent through Weston 13 (X11 backend) nested in Xvfb, at 1x and 2x, light and dark, normal and Compact view.
  - 22 of 23 passed in all 8 combinations. The miss: after a dialog closes under Weston, no window gets the keyboard back until a click. That happens on unchanged main too, and nothing here changes it.
- **Pictures.** Taken with tools/native_capture.py in its bubblewrap sandbox, with the fictional demo tree: the shortcuts window, New Bitmap image, split view at 1366 px, and Settings search, at 1x and 2x, light and dark. `python3 tools/audit-public-data.py` found no issues.
- **Not run.** A Next.js build and the website checks, because nothing on the website changed. A real screen reader speaking: only the AT-SPI names were read.

## Compatibility and release impact

- No persisted data, desktop integration, dependency or license changes.
- The message template gains the new strings. zh_CN is merged with the template in its own commit.
- The pictures use only the fictional demo tree (docs/PRIVACY.md).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
