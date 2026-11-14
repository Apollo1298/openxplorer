# Title

fix(ui): Fix offline share wording, Home spacing, archive links and other rough edges

# Body

Model: claude-opus-5-5

This fixes a set of smaller problems people notice in everyday use: how an offline share is reported, how Home looks and names files, a question hidden behind the progress window, archive browsing, the view switch losing your place, the update error, the guest sign-in link, a move reported as failed after it finished, the Preview pane's text margin and a share's name changing case. It also adds Chinese translations for recently added strings, in a separate commit that can be dropped.

| Area | Before | After |
|---|---|---|
| Offline share | "This location is unavailable" showed the raw system error ("Software caused connection abort", "Failed to mount Windows share: Connection refused"), with the text selected | "\\\\server\\share is unavailable. Make sure you are connected to the network and the server is turned on, and then try again." The system's error is under a folded Details, nothing is selected, and Try again stays |
| Home: file names | Recent and Favorites showed "plan.pdf" while the file list shows "plan" | Recent and Favorites follow View > Show > File name extensions, as the file list does |
| Home: spacing | In Compact view Quick access tiles touched (the pin sat on the next tile's name); an empty band about a row tall sat between Recent's column titles and the first row | Compact tiles are 54 pixels tall and stand apart; the first row comes right under the titles in both views |
| Move by copying | "Move by copying?" opened behind the progress window, so only its buttons showed | The question is the progress window's modal child and the progress window shows first, so the question is always in front |
| TAR links | Browsing hid a safe link (bundle/lib/libx.so -> libx.so.1) and said "1 unsafe names or links are hidden." | A relative link that stays inside the archive is listed, by the same rule extraction uses; other links and special files stay hidden; one hidden member reads "1 unsafe name or link is hidden." |
| View change | End, then Ctrl+Shift+2, jumped to the top of the list | The item with the keyboard, else the first selected item, stays in view and keeps the keyboard |
| Archive name | The tab said "Bundle.tar.gz", the address bar "Bundle.tar" | The tab, window title and address bar all use the real file name |
| Update error | "Installed: 2.0.17 Could not check for updates. GitHub could not check for updates (HTTP 403). Try again later." | "Installed: 2.0.17" on its own line, then "Couldn't check for updates (HTTP 403). Try again later." |
| Connect as guest | Looked like plain text right under the keyring note | Drawn as a link in the accent color, 16 pixels under the note. The dialog's style rules never matched; the close and eye buttons now get their intended look too |
| Finished move | A move whose copy and source removal had finished reported "Interrupted Action ... Error deleting file" when the server stopped right then | The error came from removing the empty private staging folder on the share. The move is now reported as done, with no question or "Incomplete staging folder" notice |
| Preview pane | Text started at the pane's top-left edge | Text stands 12 pixels off the edges, as document pages do |
| Share name | The address bar said "Studio", the navigation pane "studio on 127.0.0.1" | The navigation pane spells the share as the browsed address does: "Studio on 127.0.0.1" |
| TAR dates | Members with time 0 showed "1/1/1980 12:00 AM" | They show no date |
| Chinese | Home, the tab and navigation pane settings, Choose Details, Update and several dialogs were in English | 128 strings translated (separate commit) |

Left unchanged on purpose:

- One file showing "2 KB" in the Size column and "2.00 KB" in the status bar matches Windows, which writes the Size column in whole kilobytes and the status bar, Properties and tooltips with three significant digits.
- "This folder is empty." draws the item focus ring while the keyboard is on the empty list. Keyboard focus stays on the list, and the ring follows the existing focus design for an empty list.
- Split view at 1x: both panes clip their last columns and both show the same horizontal scroll bar when the pointer moves over them, on the previous build too. Column fitting was not changed.

How it was tested:

- A failing test was written first for each fix and run on the previous main, where it fails; it passes with the fix.
- `cargo fmt --check`, Clippy with the workspace's pedantic lints, `native/tools/i18n.py check`, `native/parity/check.py` and the i18n tool tests pass.
- The full workspace test suite ran in an isolated session on this branch (4,447 passed, 118 failed) and on the previous main with only the new tests added (4,429 passed, 136 failed). Compared by test name, the only differences are the new tests and the Home and archive-message tests whose expectations this change updates, which fail on main, and one flaky search-history test that also fails on main when run alone. The 115 other failures are the same on both and need a real pointer, the retired Python app or administrator mounts that this environment lacks.
- `native/tools/check.py` passes its parity, translation, driver, icon, rustfmt and Clippy stages and then stops at `location_fixture_drift`, which needs Node.js and an older Python here and fails the same way on main.
- Each fix was checked in the release build with real keyboard and pointer input on a private X11 display, and the visual ones also under Weston (Wayland) at 2x in the dark theme and Compact view. The offline share, the guest link and the share name were checked against a local Samba share with fictional files.
- Pictures were taken with the repository's capture module and the fictional demo tree, at 1x and 2x, light and dark.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
