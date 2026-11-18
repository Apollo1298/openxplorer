# Title

fix(ui): Match Windows 11's Small icons, file menu, Properties and status bar

# Body

Model: claude-opus-5-5

## Summary

Five differences from Windows 11 File Explorer (build 26200, VM check of 2026-11-18, overnight review 4) are fixed. Each was written test-first and checked with real xdotool input in a private Xvfb session.

- **Small icons (Ctrl+Shift+4).** Each 16 px icon now sits left of its name. Items fill a column top to bottom and then flow into the next column, which scrolls sideways, as List does. The columns are narrower than List's (names cut at 20 characters instead of 28) and 4 px apart. Before, Small icons was a grid of tiles with a tiny icon above each name and wide gaps. Up and Down move along the column. In the grid they jumped a whole column, so that path now leaves Small icons to GTK's own keys, as List does.
- **File right-click menu.**
  - **Add to Favorites** now shows on every file's menu, compact and classic, after Open with, as in Windows 11. It changes to **Remove from Favorites** once the file is a favorite. Before, it showed only while "Open Quick access on Home" was on, which is off by default, so the review never saw it. Favorites are saved whatever Home shows, and Home's Favorites list shows them once Quick access is on.
  - **Edit in \<default text editor\>** now shows for text files, with the editor's own icon, in its own group after Properties and before Show more options, as Windows shows "Edit in Notepad". In the classic menu it comes right after Open. Before, the only editor rows were "Open in" Visual Studio Code, Code Insiders and VSCodium (`EDITOR_DESKTOP_IDS`). With none of them installed, a .txt whose default app is LibreOffice Writer had no row. The editor is now the default app for the file's type or for `text/plain` when that app is a text editor (`TextEditor` in its desktop file's Categories). Otherwise it is the first text editor registered for `text/plain`. Writer has no `TextEditor` category, so the .txt offers the text editor (Mousepad in the test session). The Visual Studio Code rows stay. An editor that is both is listed once.
- **Properties > General.**
  - "Opens with:" shows the app's 16 px icon before its name. An app without an icon gets the generic app glyph. Both follow a change made with Change....
  - Created, Modified and Accessed stay on one line, for example "Wednesday, November 18, 2026, 8:36:06 PM". The dates' label column now takes only the room its labels need, and the gap is 8 px instead of 14. A no-break space keeps "PM" with its time if a date is ever too long. The window keeps its fixed size, and fonts stay at 12 px.
- **Properties > Details.** Under "File" the labels are now "Type" and "File location", and the order is Windows': Name, Type, File location, Date created, Date modified, Size, Attributes, Owner, Computer. Attributes now gives "A" for a normal file, "RA" for a read-only one, "HA" for a dotfile and "RHA" for both. Before, a normal file had no Attributes row. A new Computer row reads "\<host name\> (this PC)".
- **Status bar.** With items selected, the bar now ends with a divider: "47 items | 1 item selected 4 bytes |".

Left out as asked: Share, Share with, the "Remove Properties and Personal Information" link and the Advanced attributes button. The owner's extras (Open as administrator, Folders first, Select items matching, Refresh/Paste/Undo, Split view and text size rows) are unchanged. No new dependency. The desktop file's Categories are read with `glib::KeyFile`, because `gio::DesktopAppInfo` needs the gio-unix crate. `native/po/*.po` is untouched; `openxplorer.pot` was regenerated.

## Before and after

| Case | Before (2.0.20) | After | Windows 11 (VM) |
|---|---|---|---|
| Small icons | Grid, icon above name, wide gaps | Icon left of name, columns filled top to bottom, 4 px gaps | Icon left of name, columns, narrow gaps |
| Right-click a .txt (default app LibreOffice Writer) | No Add to Favorites (option off), no Edit in row | Open, Open with >, Add to Favorites, Compress to >, Copy as path, Properties, divider, Edit in Mousepad, divider, Show more options | Same, with Notepad, plus Share and Share with |
| Opens with | "LibreOffice Writer" | Writer's icon, then "LibreOffice Writer" | App icon and name |
| Modified | "Wednesday, November 18, 2026, 8:36:06" with "PM" on the next line | One line | One line |
| Details > File | Name, Item type, Folder path, dates, Size, Owner | Name, Type, File location, dates, Size, Attributes "A", Owner, Computer "vm (this PC)" | Name, Type, File location, dates, Size, Attributes "A", Owner, Computer |
| Status bar, one file selected | "47 items \| 1 item selected 4 bytes" | "47 items \| 1 item selected 4 bytes \|" | "15 items \| 1 item selected 4 bytes \|" |

## CHANGED OWNER DECISIONS

- HOME-014: Add to Favorites was offered, and Favorites changed, only while "Open Quick access on Home" was on. It is now offered on every file's menu and always saved, as in Windows 11. Tests `a_files_menu_adds_it_to_favorites_while_home_shows_them`, `a_menu_left_open_adds_no_favorite_once_the_option_is_off` and `favorites_change_only_while_home_shows_them` were replaced by `a_files_menu_adds_it_to_favorites` and `favorites_change_whatever_home_shows`.
- OPEN-015: besides "Open in" a code editor, a text file offers "Edit in" its default text editor.
- PROP-034: File rows renamed and extended (Type, File location, Attributes with A, Computer).
- PROP-003: the dates' label column is no longer aligned with the other groups' label column. It takes only the room "Created:", "Modified:" and "Accessed:" need, so the value column gets the rest.

## Tests

New, each failing before its fix, passing now:
- `window::tests::view_styles::small_icons_put_the_icon_left_of_each_name_down_columns`
- `window::tests::selection::arrows_move_down_columns_in_small_icons`
- `window::context_menu::entries::tests::a_text_file_offers_edit_in_its_default_text_editor`
- `window::tests::context_menus::a_text_file_that_opens_in_a_word_processor_offers_edit_in_the_text_editor`
- `app_catalog::the_default_text_editor_is_a_text_editor_even_when_a_word_processor_opens_text` (ox-core)
- `window::tests::home_page::a_files_menu_adds_it_to_favorites`
- `settings::tests::preferences::favorites_change_whatever_home_shows` (ox-core)
- `window::tests::properties_width::general_keeps_each_date_on_one_line_and_shows_the_apps_icon`
- `properties::general_tab::tests::pm_stays_with_its_time`
- `properties::details_rows::tests::the_file_rows_follow_windows_words_and_order`
- `properties::details_rows::tests::attributes_read_as_windows_letters`
- `window::tests::environment::the_status_bar_counts_this_pc_and_a_folder` (extended with the trailing divider)

Checks run in this container. Tests run as root here, and the Rust 1.92 toolchain was used because the container's 1.97 Clippy flags code this branch doesn't touch:
- `python3 native/parity/check.py`: passed.
- `python3 native/tools/i18n.py check`, the parity and tools unit tests (31 and 124), the drawn-icon guard, `cargo fmt --check` and `cargo clippy --workspace --all-targets -D warnings`: passed. These are check.py's own steps, run through check.py.
- `python3 native/tools/check.py`: stops at its first failing test executable, `display_scale`. That executable fails the same way on unchanged main: a launched app refuses to run as root.
- So every test executable was also run under check.py's isolation, continuing past failures, on this branch and on unchanged main 05831ab6. ox-app: 2575 passed and 12 failed on this branch, vs 2568 passed and 12 failed on main. ox-core lib: 1292 passed and 1 failed on both. Every other executable passed, except 8 failures in 6 of them, the same on both. The failures on both sides are the same tests: permission and root cases (read-only folders, no Trash, administrator prompts, the root launch refusal) and `location_fixture_drift` / `network_python` against the Python app sources. No test fails only on this branch. Doctests pass.
- Real input (xdotool) under a private Xvfb display (1366x768, 1x) with openbox and xcompmgr, as the unprivileged `nobody` user with a throwaway HOME and fictional files, LibreOffice Writer as the .txt default, the 12-hour clock, light and dark themes:
  - Ctrl+Shift+4 showed the icons left of the names down columns. Down moved from "Budget 2026" to "data", Right to the next column.
  - Right-click showed Add to Favorites and Edit in Mousepad. A click on Add to Favorites turned it into Remove from Favorites. A click on Edit in Mousepad started `mousepad` on the file.
  - Alt+Enter showed the Writer icon and one-line dates. A click on the Details tab showed the new rows.
  - The status bar showed the trailing divider.

## Not tested

- Wayland, 2x scale and Compact view (X11 1x only).
- The exact pixel width of Windows' Small icons columns was not measured in the VM. 20 characters was chosen to be narrower than List's 28.
- Orca speaking. The new icons are presentation-only, and rows keep their names.
