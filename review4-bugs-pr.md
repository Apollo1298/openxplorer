# Title

fix(ui): Fix review 4 bugs in the address bar, Extract all, large text, This PC, menus and screen-reader names

# Body

Model: claude-opus-5-5

## Summary

Thirteen bugs from overnight review 4 of 2.0.20 (upstream main 05831ab6) are fixed. Each fix has a test that failed before it, and the visible ones were checked with real mouse and keyboard input on a private Xvfb display (and Weston for Wayland).

| # | Bug | Fix |
|---|---|---|
| 1 | Alt+D or Ctrl+L while editing the address left the address bar, so the next keys went to the file list and Enter could open a file | Alt+D, Ctrl+L and F4 always select the address bar with all its text selected, as Microsoft's list says ("Alt+D: Select the address bar"). Editing ends only with Enter, Esc or focus moving away |
| 2 | Extract all wrote every file as 0600 with the time it was written, and the new folder as 0700 | A finished file gets 0666 less the umask and the modified time its archive entry records (ZIP, 7z and TAR); the folder gets 0777 less the umask just before it is published |
| 3 | At 200% text the Details pane kept its 262-pixel width and values wrapped a few letters per line | The pane's width grows with the text, as the navigation pane's does, within the room the list keeps; names no longer wrap, so the names column is as wide as its widest name |
| 4 | After 200% text, a folder opened, then Ctrl+0, Size stayed at its 200% width and pushed the other columns left | A Size column still at its default follows the list's font down as well as up; a width the user dragged stays |
| 5 | Opening This PC printed "GtkGizmo (progress) reported min width -2" | The capacity bar's trough draws its outline as an inset shadow, so its fill needs no negative margin; it looks the same |
| 6 | This PC opened with Local Disk selected | It opens with the keyboard on the first tile and nothing selected ("1 item"), as Windows 11's does |
| 7 | "27.3 GB free of 251" with "GB" on the next line | The Explorer-style sizes (status bar, Details pane, tooltips, This PC, Properties, Size column) join number and unit with a no-break space; translated patterns are unchanged |
| 8 | Settings search showed settings holding each word anywhere ("status bar" found the Update button row, "hidden files" found Indexed folders) | A search of several words looks first where settings are named (titles, keywords, control labels, headings); when any setting names every word, settings that only describe them are left out. Otherwise descriptions are searched as before |
| 9 | Tab from the file list stopped on an unnamed scroll pane | The Details pane's scroller is named "Details pane" or "Preview pane", in a role that keeps the name (GTK drops names on the generic role) |
| 10 | On Wayland the View menu ran off the bottom of a 768-pixel screen | On Wayland a menu fits the room down to its window's bottom edge and scrolls past it; in a maximized window, down to the monitor's bottom less room for a panel. X11 is unchanged |
| 11 | The navigation pane's Recycle Bin menu cut its row to "Hide section "Recent files and Recycl…" | Menus grow to their longest row, up to 560 pixels; past that a row still ends in an ellipsis |
| 12 | The shortcuts window listed "Back", "Forward" and "HomePage" | They read "Browser Back key", "Browser Forward key" and "Browser Home key" |
| 13 | The file list's accessible name was the typing hint | It is named "Items View", as Windows names Explorer's list; the hint is its description only |

## CHANGED OWNER DECISIONS

- NAV-028 ("Pressing Ctrl+L again returns to breadcrumbs", from Dolphin) is now `n-a`: Windows' Alt+D always selects the address bar, and the toggle let keys reach the file list. NAV-026 gains the new rule and test.
- ARC-018: extracted files were owner-only (0600) and the published folder 0700. Files are still written owner-only inside a 0700 staging folder; once complete, they get 0666 less the umask and the archive's modified time, and the folder 0777 less the umask. The archive's own modes are still never applied.

## NOT CONFIRMED

- Whether Windows 11's Extract all keeps each file's modified date from the archive (the review expects it; no Microsoft page found).
- Whether Windows 11's Details pane widens with text size, and whether its Tab order stops in the Details pane at all.
- Whether Windows 11's This PC focuses the first tile without selecting it (the review's VM shows nothing selected, "2 items").

## How it was tested

- New or changed tests, each failing before its fix: `address_input.rs` (`ctrl_l_and_alt_d_again_keep_editing_with_everything_selected`), `archive_gio.rs` (ZIP and 7z through the GIO output: modes and times), `accessibility.rs` (`columns_of_a_folder_opened_at_large_text_shrink_after_ctrl_0`, `the_file_list_is_named_items_view_and_described_by_the_hint`), `environment.rs` (`this_pc_draws_its_capacity_bars_without_gtk_warnings` with a new recorder of GTK's warnings, `this_pc_opens_with_the_first_tile_focused_and_nothing_selected`), `explorer_text.rs` (`numbers_and_units_are_joined_by_a_no_break_space`), `settings_page/tests.rs` (`several_words_find_the_settings_they_all_name`), `details_pane.rs` (`the_panes_scroller_is_named_as_the_pane`), `menu_popover/height.rs` (`on_wayland_a_menu_fits_the_room_beside_its_anchor`), `sidebar.rs` (`the_recycle_bin_menu_is_as_wide_as_its_longest_row`), `shortcuts_window.rs` (`browser_keys_read_in_words`), `breakpoints.rs` and `panes_layout.rs` (the Details pane at 200%).
- Real input with xdotool on the release build, as an unprivileged user, on a private 1366x768 Xvfb at 1x with Openbox and xcompmgr and fictional files: Alt+D, Alt+D, typing; Ctrl+L while editing; the review's "can't find" sequence (nothing opens); Extract all of a ZIP and a 7z (`stat` shows `-rw-r--r--` and the archive's 20:36 time, folders `drwxr-xr-x`); 200% text with the Details pane; Ctrl+0 after opening Documents at 200%; This PC (no Gtk-WARNING in the log, first tile focused, "1 item"); the drive's space in the Details pane; the Recycle Bin menu.
- Wayland: Weston (x11 backend, default panel) nested in a private Xvfb at 1366x768, the window resized and moved lower but fully on screen: 2.0.20's View menu runs off the screen; with the fix it ends at the window's bottom and scrolls to "Reset text size".
- AT-SPI (pyatspi) on the release build: the file list reads `tree table 'Items View'` with the hint as its description, and Tab from the list lands on `scroll pane 'Details pane'`.
- Checks, with Rust 1.92.0 as CI pins it: rustfmt, Clippy with the workspace lints, `python3 native/parity/check.py`, the i18n template check, `tools/sync-docs.py --check` and `tools/audit-public-data.py` pass. `python3 native/tools/check.py` stops at its first failing test binary, and here `display_scale` fails on unchanged main too, because the app refuses to start as root, the only user the test container runs as. So every test binary was run in the same isolation (private Xvfb, D-Bus session and HOME, the Python app's sources) without stopping: 4,781 passed and 135 failed, and all 135 also fail on unchanged 05831ab6 in this environment (89 file-dialog tests that cannot start their private bus, file-permission tests that root passes, the D-Bus service and portal tests, `location_fixture_drift`, and launches refused as root).
- The website's pictures were not changed; no website build was run.
