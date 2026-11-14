# Title

feat(ui): Match nine more Windows 11 File Explorer behaviors

# Body

Model: claude-opus-5-5

This brings nine behaviors in line with Windows 11 File Explorer. Five were checked on Windows 11 (build 26200) on 2026-11-14, three follow Microsoft's documentation, and one fixes focus rings that showed up without any keyboard navigation. Quick access tiles on Home get Windows' new-style menu, wildcard searches find names that contain the pattern, the standard folders' tabs show their navigation pane glyphs, column titles lose their tooltip, the Rename File question uses Windows' words, Backspace in Open and Save dialogs goes up a level, Alt while dragging makes shortcuts, Compress to can make 7z archives, and blue focus rings appear only after keyboard navigation.

## Changes

| Change | Before | After | Windows |
|---|---|---|---|
| Quick access tile menu on Home | A plain list (Open, Open in new tab, Open in new window, Open in Terminal, Open folder with…, Cache this folder for search, Unpin from Quick access, Previous versions, Properties) | Windows 11's menu: Copy in the icon strip, then Open (Enter), Open in new tab, Open in new window, Unpin from Quick access, Compress to ›, Copy as path (Ctrl+Shift+C), Properties (Alt+Enter), Open in Terminal, Show more options. Show more options opens the full classic menu, which keeps the app's own items. Right-click, Menu and Shift+F10 all open it. The keyboard shows only as the first row's highlight, never as a ring around the whole list, and the tile gets the keyboard back when the menu closes. Ctrl+Shift+C on a tile copies its path. | Same list on Windows 11, plus Pin to Start, which is Windows-only and left out |
| Wildcard search words | `a?b` had to match the whole name, so it found nothing in a folder with a1b.txt and acb.txt | A wildcard word is found within the name, as a plain word is: `a?b`, `?cb` and `a*b` find a1b.txt and acb.txt. `a?b.txt` and `*` work as before. As with plain words, a match must start before the extension or be the whole extension. Indexed and live searches use the same rule. | `a?b` finds a1b.txt and acb.txt |
| Tab icons | Downloads, Documents and the other standard folders showed a yellow folder, as did the home folder (when "Open Quick access on Home" is off) | Desktop, Downloads, Documents, Pictures, Music and Videos show their navigation pane glyphs (on the network bar when the folder is on the network), and the home folder shows Home's glyph | The Downloads tab shows the green Downloads glyph |
| Column title tooltip | "Drag to resize · double-click to fit loaded items (up to 2,000)" | No tooltip. Resizing, double-click to fit and the keys are unchanged, and screen readers still get the description. | No tooltip |
| Renaming to a taken name | A dialog named the existing file: "Would you like to rename "Notes 1.md" to "Notes 1 (2).md"?", with both files' sizes and dates | A compact message box titled "Rename File": "Do you want to rename "Notes 2.md" to "Notes 1 (2).md"?" and on the next line "There is already a file with the same name in this location.", with Yes (Enter, Alt+Y) and No (Esc, Alt+N). Yes renames, No goes back to editing. | Same wording and buttons |
| Backspace in Open/Save dialogs | Went Back, and did nothing with no history | Opens the folder one level up; at / it does nothing, and in File name it deletes text | "Backspace: Open a folder one level up if a folder is selected in the Save As or Open dialog box." ([Keyboard shortcuts in Windows](https://support.microsoft.com/en-us/windows/keyboard-shortcuts-in-windows-dcc61a57-8ff0-cffe-9796-cb9706c75eec)) |
| Alt+drag | Copied (X11) or asked (GNOME on Wayland) | Alt at the drop of OpenXplorer's own items creates shortcuts, using the same code as "Create shortcuts here". Ctrl still copies, Shift moves, and a plain drag moves within a drive. The drag label says "Create link in …". | "Alt+mouse drag a file: When you drop the file, create a shortcut to the original file in that location." ([Keyboard shortcuts in Windows](https://support.microsoft.com/en-us/windows/keyboard-shortcuts-in-windows-dcc61a57-8ff0-cffe-9796-cb9706c75eec)) |
| Compress to › 7z File | Compress to offered ZIP File, TAR File and Additional options | Compress to also offers 7z File, in the file menu, the navigation pane's pinned folders and Home's tiles, and the Create archive dialog has a 7z format (LZMA2). Archives go through the same pipeline and safety rules as ZIP and TAR: links aren't followed, no name is replaced, progress and Cancel work, a cancelled archive is removed, and Undo works. Opening and extracting 7z archives isn't supported yet. | "Support for creating 7-zip and TAR archives" ([What's new in Windows 11, version 24H2](https://learn.microsoft.com/en-us/windows/whats-new/whats-new-windows-11-version-24h2)) |
| Focus rings | GTK turned the window's focus rings on for any Ctrl or Alt shortcut (Ctrl+C, Alt+PrtSc) and never off on a click, so the navigation pane's Home row, the address bar's crumbs and chevrons and other controls kept a blue ring although only the mouse was used. F6 showed no ring. | No ring at startup, after a click or touch, or after a shortcut that moves the keyboard nowhere. Tab, F6, the arrows and Home/End/Page keys show it, and the next click hides it again. Items keep their 1 px outline after a click, as before. | Focus rectangles only after keyboard navigation |

Not changed:

- **Home as the default start.** Microsoft says File Explorer opens to Home with Quick access by default ([File Explorer in Windows](https://support.microsoft.com/en-us/windows/experience/fileexplorer/file-explorer-in-windows)). "Open Quick access on Home" stays off, the owner's choice.

## New dependency

`sevenz-rust2` (Apache-2.0, pure Rust) writes the 7z archives. It's pinned at `=0.20.2` with `default-features = false` and only the `compress` feature: no encryption, PPMd, BZip2 or other codecs. 0.20.2 is the last release that builds with the workspace's `rust-version` of 1.92 (0.21 and later need 1.93). It brings `lzma-rust2` 0.16.5 (Apache-2.0) next to the 0.25 already in the tree. `cargo deny` found no advisories for either crate, and their licenses are allowed. Cargo.lock and the Flatpak `cargo-sources.json` are updated, and the packages install each crate's license from Cargo.lock as before.

## CHANGED OWNER DECISIONS

Where an earlier choice differed from Windows 11, Windows' behavior is now the default. Parity notes and BACKLOG.md list each one:

- HOME-013: Quick access tiles used the pin's plain list. They now use Windows 11's menu.
- SRCH-004: wildcard words used to match the whole name, as in Dolphin. They are now found within the name, so `*.pdf` also finds `bank.pdf.txt` and `??` finds every name of two or more characters.
- TAB-010: standard folders and the home folder used to show a yellow folder in their tabs.
- VIEW-029: column titles used to have app.js's resize tooltip.
- OPS-010: the Rename File question used to name the existing file and show both files' details.
- NAV-004: Backspace used to go Back in Open and Save dialogs too.
- DND-017: Alt at a drop used to copy, or ask on GNOME Wayland.
- ARC-023: Compress to offered no 7z.
- ACC-009: the focus ring followed GTK's defaults (on after any Ctrl/Alt shortcut, never off on a click, not on for F6).

## NOT CONFIRMED

These are guesses or open points that still need a check on Windows 11:

- Whether `*.pdf` also finds `bank.pdf.txt` on Windows, and whether `??` finds every name of two or more characters. Both follow from "found within the name, as a plain word is".
- Whether a wildcard match may start inside the extension. The app applies its plain-word rule: it must start before the extension or be the whole extension.
- The Rename File box's icon (the app shows its warning icon), whether Windows shows extensions in the question when they are hidden, and what Windows asks when a folder is renamed to a taken name. Folders keep the old refusal message here.
- Where the separators go in the tile menu, what Windows' "Show more options" lists for a pinned folder, whether Esc returns focus to the tile, and whether Ctrl+Shift+C works on a tile.
- Which glyph Windows shows on a tab of a standard folder that lives on a network share.
- Whether the item check box in the Details header ("Select all") has a tooltip on Windows. It keeps its tooltip here.
- Backspace in an Open/Save dialog with nothing selected. Microsoft's wording says "if a folder is selected", and the app goes up either way.
- Windows' order of ZIP File, 7z File and TAR File in Compress to, and of the formats in its Create archive dialog.
- Alt+drag of items coming from another app. The app keeps what that drag settled on.
- Whether Windows shows focus visuals after a shortcut that moves the keyboard to a new place, such as Ctrl+Shift+E (navigation pane) or Alt+D (address bar). Here no ring is drawn for those until Tab, F6 or an arrow is pressed.

## How it was tested

Each change was written test-first. Each new test failed on unchanged main (or, for 7z, before the writer existed) and passes now:

- Quick access tile menu: `a_quick_access_tiles_menu_is_windows_11s`, `a_quick_access_tiles_menu_draws_no_ring_around_its_rows`
- Wildcards: `a_wildcard_word_matches_within_the_name_as_a_plain_word_does`, `a_wildcard_word_matches_within_the_cached_name`, `a_wildcard_word_finds_names_that_hold_it`
- Tab icons: `a_standard_folders_tab_shows_its_navigation_pane_glyph`
- Column titles: `column_titles_show_no_tooltip`
- Rename File question: `a_file_renamed_to_a_taken_name_is_offered_the_next_free_number`
- Backspace in dialogs: `backspace_in_the_file_list_opens_the_folder_one_level_up`
- Alt at a drop: `alt_at_the_drop_makes_shortcuts_to_own_items`
- 7z: `a_7z_name_writes_a_7z_archive_with_the_same_items` and `a_7z_cancelled_while_it_writes_leaves_nothing_behind`, which read the archive back
- Focus rings: `the_focus_ring_shows_only_after_keyboard_navigation`, with real XTEST keys and clicks

Checks that pass: `python3 native/parity/check.py`, `rustfmt --check`, Clippy with `-D warnings` (workspace lints and CONTRIBUTING's pedantic flags), `native/tools/i18n.py check`, `tools/sync-docs.py`, `tools/audit-public-data.py`, and `cargo deny check advisories licenses` for the new crates.

Every test binary and the doctests ran with `native/tools/check.py`'s isolation (private Xvfb display, private D-Bus session, disposable home), on this branch and on unchanged main:

- `ox_app`: 2352 passed on this branch.
- Failures are the same tests by name on both: the `admin_copy` and `admin_delete` tests (they depend on this container's mounts), two Python-fixture comparisons (they depend on the host Python's URL parsing), and two thumbnailer sandbox tests (they depend on bubblewrap in this container).

Manual checks used real input (xdotool) on Xvfb (X11, 1x, light, normal and Compact view) and on weston inside Xvfb (Wayland, 2x, dark, Compact), with fictional files:

- The tile menu by right-click, the Menu key and Shift+F10 (Shift+F10 on X11 only), and each of its items. Compress to › 7z File made an archive that p7zip 16.02 lists and tests.
- Wildcard and plain searches, with and without contents search, including subfolders.
- Tab icons for each standard folder, a plain folder, Home, This PC and the Recycle Bin.
- Column title hover and drag-resize.
- Renaming a file and a folder to a taken name, including Yes, No, Enter and Esc.
- Backspace in a Save dialog opened through the portal interface: in File name, in a folder, and at /.
- Alt, Ctrl and plain drags of files and folders onto folders and onto the pane.
- Focus rings at startup and after clicks, Ctrl+C, Tab, F6 and arrows.

Each Alt drag on X11 logs `gdk_drop_set_actions: assertion '(actions & GDK_ACTION_ASK) == 0' failed`. Unchanged main logs the same, so it comes from GTK.

This wasn't run on a real desktop, against network shares, or with a screen reader.

Translations: the new Rename File message has no zh_CN translation yet, so it shows in English there.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
