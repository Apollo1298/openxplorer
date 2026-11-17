# Title

fix(ui): Explain a mistyped address, count This PC, preview RTF text and keep tabs, large text and new files right

# Body

Model: claude-opus-5-5

## Problem and resulting behavior

Seven things a person coming from Windows would notice early:

- **A mistyped address showed GIO's error.** Typing `/home/demo/Documnets` and pressing Enter showed a toast with "Error when getting information for file …: No such file or directory". It is now answered as Windows 11's File Explorer answers it: a small message box titled "OpenXplorer" with the red error icon, "OpenXplorer can’t find ‘/home/demo/Documnets’. Check the spelling and try again." and one OK button, the default. After OK the address bar holds the typed text again, ready to correct. Windows 11 (build 26200) shows a "File Explorer" box with "Windows can't find '…'. Check the spelling and try again." and OK; no Microsoft documentation page with this wording was found, so the wording is from that Windows check, not from Microsoft docs. The box uses the same compact message box as Rename File, with Fluent's `dismiss_circle_32_color` icon added from the pinned Fluent UI System Icons 1.1.343 package.
- **New files were private.** A text document, bitmap, Rich Text Document, ZIP or template copy made with New was `-rw-------`. It is now created as other apps create files, 0666 less the umask (0644 or 0664). The file is still written to a private hidden stage first and published without overwriting.
- **The Preview pane showed an RTF file's markup.** It now shows the document's text: paragraphs and line breaks, tabs, accented and Unicode characters, without the font table, colors or other hidden groups. Bold, italics and fonts are not drawn. No dependency was added.
- **This PC said "Ready" and "This PC (0 items)" beside a yellow folder.** The status bar now counts the tiles ("1 item", "1 item selected"), and the Details pane shows This PC's own icon with "This PC (1 item)", or a selected drive's icon, name and space.
- **At 200% text, columns and the navigation pane kept their 100% widths**, so dates, types and folder names were cut short. Default column widths and the navigation pane's default width now grow with the text size and shrink back with it. Widths a person set stay as they are. Name keeps room for names, and the list scrolls sideways when the window is too narrow.
- **"Reopen my tabs" lost tabs when the app was ended by a signal**, for example SIGTERM at logout: tabs were saved only when the last window closed. The window in front now saves its tabs every 3 seconds when they changed, off the main thread, and at once when the desktop says it is logging out or shutting down (GtkApplication's `query-end`). A signal loses at most the last 3 seconds. There is no handler that saves at the signal itself: in glib-rs 0.22, GLib's own signal source (`g_unix_signal_add`) is bound only in the separate `glib-unix` crate, which would be a new dependency, and `unsafe` code is not allowed.
- **A Chinese sentence had English in the middle** ("ZIP 打开使用 the desktop choice，…"). The sentence for "the desktop choice" is now one whole message, and its Chinese translation reuses the catalog's own wording; a translator should still check it.

"Use the desktop font" was also reported as springing back off. That is already fixed on main by 0176e963; it was checked again with real clicks and `desktopFont` reaches settings.json.

| | Before | After |
|---|---|---|
| Mistyped address | Toast: "Error when getting information for file “…”: No such file or directory" | Message box "OpenXplorer", error icon, "OpenXplorer can’t find ‘…’. Check the spelling and try again.", OK; the typed text stays in the address bar |
| New > Text Document (umask 0002) | `-rw-------` | `-rw-rw-r--` |
| Preview of an RTF file | `{\rtf1\ansi\deff0{\fonttbl…` | The document's text |
| This PC status bar | "Ready" | "1 item", "1 item selected" |
| This PC Details pane | Yellow folder, "This PC (0 items)" | This PC's icon, "This PC (1 item)"; a selected drive's icon, name and space |
| 200% text | Columns 176/135 px, navigation pane 210 px, text cut short | Columns 282/270 px, navigation pane 420 px, Name at least 280 px |
| SIGTERM with "Reopen my tabs" on | Tabs from the previous normal close | The tabs open at the time, give or take 3 seconds |
| zh_CN, ZIP line | "ZIP 打开使用 the desktop choice，…" | "ZIP 打开使用桌面环境的默认应用。…" |

## Validation

- A failing test for each fix, run on unchanged main first: all seven failed there and pass with the change.
  - `a_mistyped_address_says_it_was_not_found_in_a_message_box`
  - `a_new_document_gets_the_umask_mode_not_the_private_stage_mode`
  - `the_preview_pane_shows_a_rich_text_documents_text_not_its_markup`
  - `this_pc_counts_its_tiles_in_the_status_bar_and_the_details_pane`
  - `large_text_widens_default_columns_and_the_navigation_pane`
  - `the_open_tabs_are_saved_while_the_window_is_open`
  - `test_the_zip_line_without_a_handler_is_one_whole_chinese_sentence`
- New unit tests: `rtf_markup_becomes_its_text`, `unicode_fallbacks_and_escapes_are_read_as_characters`, `a_new_rich_text_document_has_one_empty_paragraph` and `default_widths_grow_with_the_text`.
- `python3 native/tools/check.py` with Python 3.12 (the container's Python 3.13.18 splits `smb://[V1.x]/` differently from the committed fixture, so `location_fixture_drift` fails there on main as well): the parity inventory, the driver's 124 tests, the drawn-icon guard, rustfmt and Clippy pass. The ox-app tests ran 2,537: two sandboxed-thumbnailer tests fail, and fail the same way on unchanged main, because this container cannot apply the sandbox's limits. The ox-core unit tests fail nine `admin_copy` tests, which also fail on unchanged main because of this container's temporary folder. Every other test executable and the doctests pass. Three existing tests that asserted the old behavior were updated: the two in `ops_create.rs` that expected 0600, the status bar test that expected "Ready" on This PC, and a CMD-018 test that expected the missing-address error in the toast.
- `python3 native/parity/check.py` and `python3 native/tools/i18n.py check` pass.
- Real input with xdotool on the release build, in a private Xvfb display and in Weston nested in Xvfb, with fictional files:
  - X11 at 1x light, 1x dark, and 2x dark in Compact view.
  - Weston at 1x light, and 2x dark in Compact view.
  - Checked there: the address message box (Enter presses OK and the typed text comes back), New with every template (`ls -l` gave 0664 under umask 0002, the folder 0775), the RTF preview, This PC with and without a selected drive, Ctrl+plus to 200% in Music, "Use the desktop font", the Chinese ZIP line, and `kill -TERM` with "Reopen my tabs" on, after which the next start reopened both tabs.
  - No crash and no Gtk-CRITICAL.
- Pictures of the address message, This PC, 200% text and the RTF preview were taken with the repository's capture library at 1x and 2x, light and dark.
- Not checked: a real desktop logout (`query-end`), and Orca.

## Compatibility and release impact

- No persisted format changes. `session.json` is written while the app runs when "Reopen my tabs" is on, in the same format.
- The app now registers with the desktop (GtkApplication `register-session`) to hear `query-end`. It does not ask to delay logout.
- No new dependencies. One icon file is added, `status/ox-dismiss-circle-32-color.svg`, copied byte for byte from the pinned Fluent UI System Icons 1.1.343 package and recorded in `SOURCES.md`.
- One new translatable message for the ZIP line, and two more for the address message and the drive's "Space" row, are in the template. The Chinese catalog gains one translation.
- Examples and pictures use only fictional files and paths (`docs/PRIVACY.md`).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
