# Overnight review 4: OpenXplorer 2.0.20 (upstream main 05831ab6)

Reviewed upstream main `05831ab6` (2.0.20, after #363) the way someone switching from Windows 11 would. This is a report only: no app code was changed and no PRs were opened.

**Rank** is how soon a Windows switcher would notice it: 1 = in the first minutes, 5 = rarely.

**Reproduced** says how each finding was checked:
- "Yes (X11)" or "Yes (Wayland)": driven with real xdotool input on the release build.
- "Code reading only": not run.

Translations were not tested, read or reported on.

## How it was tested

- **Build.** `cargo build --release --locked` of 05831ab6 (Rust 1.92), run as an unprivileged test user.
- **Isolation.** Never on a live desktop. Each session had its own Xvfb display (1366x768, 1x), its own D-Bus session and a throwaway HOME. Openbox and xcompmgr ran inside the private display so menus and dialogs draw as on a desktop.
- **Wayland.** Weston (x11 backend, 1x) nested in a second private Xvfb display, for a short check of the address bar, submenus and tab restore.
- **Fictional files.** Documents holds Budget 2026.txt, Notes 1-3.md, Agenda.rtf, data.json, page.html, script.py and Projects/Sub. Pictures holds two PNGs and a Trips folder. Downloads holds Report.zip, Report.7z, Report.tar, Report.tar.gz, an encrypted Locked.7z and a header-encrypted LockedHeaders.7z, all built with 7z and tar. Big holds 2,000 small text files. Paths below are written as `~/...`.
- **Themes.** Light (System default) and Dark, switched in Settings > Appearance.
- **Screen-reader names and focus order.** Read through AT-SPI (pyatspi).
- **Checks run.** `python3 native/parity/check.py`: passed. `native/tools/check.py` and the cargo test suite: not run.
- **Not tested.** Split view, network shares, removable media, Gallery, drag and drop, PDF previews (no bubblewrap) and Orca speaking.
- **Environment limits, left out.** Dialogs open where Openbox puts them. Weston's window has only a close button. LibreOffice is the desktop's default for .txt here, so "open" lands there.

## Bugs

| Rank | What happens | Exact steps | What Windows 11 does | Suggested fix | Reproduced |
|---|---|---|---|---|---|
| 2 | **Alt+D and Ctrl+L leave the address bar when it is already being edited, so the next keys go to the file list.** Typing then jumps the selection ("Jump to: Notes - Notes 1.md" in the status bar) and Enter opens that file in another app. It is easy to hit right after the new "can't find" box, which leaves the address bar in edit mode. | Documents. Alt+D, type `~/Documnets`, Enter. Press Enter on the "OpenXplorer can't find" box. Press Alt+D, type `Notes`, Enter: Notes 1.md opens. Shorter: Alt+D, Alt+D, type. Ctrl+L does the same. | Alt+D always selects the address bar ("Alt+D: Select the address bar", https://support.microsoft.com/en-us/windows/keyboard-shortcuts-in-windows-dcc61a57-8ff0-cffe-9796-cb9706c75eec). Pressing it again just keeps the text selected. | Make Alt+D, Ctrl+L and F4's entry point "enter edit mode and select all", never a toggle. Leave edit mode only on Esc, Enter or focus moving away. | Yes (X11 and Wayland) |
| 3 | **Extract all writes every file as mode 0600 with today's time.** The archives record 8:36 PM; the extracted files say 8:48 PM and are `-rw-------`, so the group and other users can't read them. Seen for 7z and ZIP. | Downloads > Report.7z > Extract all > Extract. Then Report.zip > Extract all > Replace. Run `ls -l` and `stat` on the extracted files. | Extract all keeps each file's modified date from the archive. Not sure about a Microsoft page (needs a Windows check). | `archive/extract/output.rs:71` creates every file with `FileCreateFlags::PRIVATE` and nothing sets the mode or time afterwards. Set the mode to 0666 less the umask (as #358 now does for New), and set `time::modified` from the archive entry before publishing. | Yes (X11) |
| 3 | **At 200% text the Details pane keeps its 1x width, so values wrap a word or a few letters per line.** Type reads "Text / Docu- / ment"; File location reads "/home/ / tester/ / ..." one folder per line. The label column takes half the pane. | Settings > Appearance > Text size 200%. Documents, Details pane on, select Budget 2026. | Not sure (needs a Windows check). | Scale the Details pane's default width with the text size, as #358 did for the navigation pane and columns, and size the label column to its widest label. | Yes (X11) |
| 4 | **Columns keep their 200% widths after going back to 100%.** Name shrinks and Date modified, Type and Size stay wide until the app restarts. | Text size 200%, open Documents, then Ctrl+0. Compare the column positions with before. | Not a Windows question. | Recompute default (not user-set) column widths when the text size changes back down. | Yes (X11) |
| 4 | **Opening This PC prints a Gtk-WARNING each time, and again when the Details pane is toggled there.** `GtkGizmo (progress) reported min width -2, but sizes must be >= 0`. | Click This PC in the navigation pane; check stderr. | Not a Windows question. | `resources/skin/landing.css:64` gives `.landing .capacity > trough > progress` `margin: -1px`. Drop the negative margin or set `min-width: 2px` on the progress node. | Yes (X11) |
| 4 | **This PC opens with Local Disk already selected.** The status bar says "1 item selected" and the Details pane shows the drive before anything is clicked. Other folders open with nothing selected. | Click This PC. | Not sure (needs a Windows check); Windows normally opens a folder with nothing selected. | Put keyboard focus on the first tile without selecting it, as in other folders. | Yes (X11) |
| 4 | **Units wrap away from their numbers.** The This PC Details pane reads "27.3 GB free of 251" with "GB" alone on the next line. | This PC, Details pane on, look at Space. | Not sure (needs a Windows check). | Join number and unit with a no-break space in the size formatter, or keep the value on one line. | Yes (X11) |
| 4 | **Properties > General wraps the time onto its own line.** Created and Modified read "Wednesday, November 18, 2026, 8:36:06" with "PM" alone below. | Select a .txt, Alt+Enter. | The peer's Windows VM check shows "Saturday, November 14, 2026, 8:54:14 AM" on one line. | Widen the dialog a little, or let the value column take the room the label column doesn't need. | Yes (X11) |
| 4 | **Settings search matches any one word of a multi-word search.** "status bar" shows "Show the Update button in the command bar". "hidden files" still shows "Indexed folders" next to "Show hidden files, folders, and drives". | Settings, type each phrase in "Search settings". | Not a Windows question. | Require every word (or rank all-word matches first and drop single-word matches when an all-word match exists). | Yes (X11) |
| 4 | **The Details pane is a Tab stop with no accessible name.** AT-SPI reads `[scroll pane] ''`; a screen reader says nothing there. | Click in the file list, press Tab once. | Not sure (needs a Windows check). | Name the pane "Details pane" (or "Preview pane"), or make the scroller itself not focusable. | Yes (X11; AT-SPI) |
| 4 | **On Wayland the View menu runs off the bottom of a 768 px screen.** "Reset text size" is cut in half and there is no scroll arrow. | Weston 1366x768, click View. | Not sure (needs a Windows check). | Let the menu flip upwards or scroll when it doesn't fit below, as it does on X11. | Yes (Wayland only) |
| 5 | **The navigation pane's Recycle Bin menu cuts off a row.** It reads "Hide section "Recent files and Recycl…". | Right-click Recycle Bin in the navigation pane. | Windows has no such row. | Widen the menu to its longest row, or shorten the row to "Hide this section". | Yes (X11) |
| 5 | **The shortcuts window shows raw key names.** "Alt+Left arrow, Back, Backspace", "Alt+Right arrow, Forward" and "Alt+Home, HomePage". | Ctrl+?, read Navigation. | Microsoft names these "Browser Back key" style keys in words. | Map XF86Back, XF86Forward and XF86HomePage to readable names ("Browser Back key", "Browser Forward key", "Browser Home key"). | Yes (X11) |
| 5 | **The file list's accessible name is an instruction, repeated as its description.** Both read "Folder contents - type a filename prefix to select". | Read the list through AT-SPI. | Windows names it "Items View". | Name it "Items" or "Folder contents" and keep the hint only in the description. | Yes (AT-SPI) |

No crashes, hangs or Gtk-CRITICALs were seen. The This PC warning above is the only Gtk message.

## Unlike Windows (Microsoft-confirmed)

**No new differences beyond Alt+D**, which is listed under Bugs with its Microsoft source because it can open the wrong file.

## Unlike Windows (VM-confirmed 2026-11-18)

A peer session checked Windows 11 build 26200 in a VM (extensions shown, a folder of .txt files, Details view, Preview pane on) and listed the menus and Properties items below. Cloud-only items (Stored locally, the Shared pill, Activity, Pin to Start, Digital Signatures) were left out as asked. Each row says what OpenXplorer 2.0.20 shows with real input.

| Rank | Where | Windows 11 (VM) | OpenXplorer 2.0.20 | Reproduced |
|---|---|---|---|---|
| 3 | Right-click a .txt | Icon row Cut, Copy, Rename, Share, Delete. Then Open, Open with >, Share with >, Add to Favorites, Compress to... >, Copy as path, Properties, divider, Edit in Notepad (the default editor), divider, Show more options. | Icon row has no Share. No "Share with >", no "Add to Favorites" and no "Edit in <default app>" row. Open, Open with >, Compress to >, Copy as path (Ctrl+Shift+C), Properties (Alt+Enter) and Show more options match. | Yes (X11) |
| 5 | Right-click a folder | Cut, Copy, Rename, Delete; Open, Open in new tab, Open in new window, Pin to Quick access, Compress to, Copy as path, Properties, Open in Terminal, Show more options. | Matches, plus an extra "Open as administrator…" row before Show more options. | Yes (X11) |
| 4 | Right-click the background | View >, Sort by >, Group by >, New >, Properties, Open in Terminal, Show more options. | Adds Refresh (F5), and grayed Paste and Undo, between Group by and New, plus "Open as administrator…". | Yes (X11) |
| 5 | "..." with nothing selected | Pin to Quick access; Select all, Select none, Invert selection; Properties, Options. | Matches (Documents shows "Unpin from Quick access" since it is pinned), plus "Select items matching…". | Yes (X11) |
| 5 | View menu | Eight views with a dot; Details pane, Preview pane (dot when on); Show >. | Matches, dots included, plus Split view, Folder tree, Terminal, Larger text, Smaller text and Reset text size. Compact view is under Show in both. | Yes (X11) |
| 5 | Sort menu | Name, Date modified, Type, More >; Ascending, Descending; Group by >. | Matches, plus a checked "Folders first" row. | Yes (X11) |
| 4 | Properties > General of a .txt | "Opens with:" shows the app's icon and name; Attributes row has an Advanced... button. | "Opens with: LibreOffice Writer" has no icon. There is no Advanced... button. Tabs, Type of file, Change..., Location, Size, divider order and OK/Cancel/Apply (Apply grayed) match. The date wrap is under Bugs. | Yes (X11) |
| 4 | Properties > Details of a .txt | Grouped under "File": Name, Type, File location, Date created, Date modified, Size, Attributes "A", Owner, Computer; link "Remove Properties and Personal Information". | Labels are "Item type" and "Folder path" instead of "Type" and "File location". There is no Attributes row, no Computer row and no Remove Properties link. Name, dates (short form), Size and Owner match. | Yes (X11) |
| 5 | Status bar with one file selected | "15 items \| 1 item selected 4 bytes \|". | "9 items \| 1 item selected 35 bytes", with no trailing separator. | Yes (X11) |

## Not sure (needs a Windows check)

| Rank | What OpenXplorer does | What to look at on Windows 11 |
|---|---|---|
| 3 | Small icons (Ctrl+Shift+4) is a grid with a tiny icon above each name and wide gaps. | Windows' Small icons puts the icon to the left of the name, in rows. |
| 4 | The RTF preview shows the text in a monospace font with no bold or fonts. | How the Preview pane renders a formatted RTF file. |
| 4 | 7z and TAR files use a plain document icon, while ZIP uses the zipped-folder icon, though all three open like folders. The Type column cuts "Compressed (zip…" and "Tar Archive (gzip-…" at its default width. | Icons and Type names for .7z and .tar in Windows 11, and the default Type width. |
| 4 | An encrypted 7z lists its files, then says "This 7z archive is encrypted, which isn't supported" on open. A header-encrypted 7z shows "This archive can't be opened", yet Extract all stays enabled and only shows the error inside its dialog. | Whether Windows 11 opens password-protected 7z, and whether Extract all is offered for one it can't read. |
| 5 | Inside an archive the right-click menu has no Properties and the search box is grayed out. | Whether Windows offers Properties and search inside a ZIP. |
| 5 | A .txt's right-click "Open with" is a submenu; a .zip's is a plain "Open with…" row. | Windows uses a submenu for both. |
| 5 | With nothing selected, the Details pane says "Select a single file to get more information." even in an empty folder and in search results ("Documents (3 items)"). | What Windows' Details pane shows for no selection and for search results. |
| 5 | The front tab's title is cut mid-letter beside its close button: "Documer". | #362 says Windows cuts at the last pixel; confirm with a long title. |

## Still broken from earlier reviews

| Rank | Finding (review) | State on 05831ab6 | Reproduced |
|---|---|---|---|
| 4 | Settings search: "hidden files" finds unrelated rows (review 3). | **Partly fixed.** "How sizes are counted" no longer appears, but "Indexed folders" still does (see the Bugs row on multi-word search). "extensions" and "language" now work. | Yes (X11) |
| 4 | Compact view is not in the View menu (review 1, Microsoft doc). | **Withdraw.** The peer's Windows 11 VM shows Compact view under View > Show, which is where OpenXplorer has it. | Yes (X11) |

Split view columns (review 2) were not retested. Every other review 3 item checked tonight is fixed (see Worked well).

## Worked well

- **Submenus on hover (#359).** View > Show opens on hover after the delay, stays open while the pointer moves into it, and its items work. Same on Wayland. (Other submenus were not hover-tested.)
- **Replace (#362).** With the default (Recycle Bin on), Replace puts the old file in the Recycle Bin and Undo brings it back. With the setting off, Replace overwrites and Undo says in a message box: ""Budget 2026.txt" was replaced, so Undo can't bring back the old file."
- **Properties name with hidden extensions (#362).** The name box and title show "Budget 2026"; typing "Budget 2027" gives Budget 2027.txt, and "Budget 2027.md" gives Budget 2027.md.txt, as Windows does. Undo reverses both renames.
- **Address bar (#358).** A typo shows "OpenXplorer can't find '~/Documnets'. Check the spelling and try again." in a message box, and afterwards the typed text is kept selected.
- **This PC (#358, #361).** The status bar counts "1 item" and "1 item selected", the Details pane shows the drive icon and space, and This PC and Properties agree on "27.3 GB free of 251 GB" with byte counts that add up.
- **RTF preview (#358).** Agenda.rtf shows its text, including a \u euro sign, not its markup.
- **Tabs (#358, #362).** 6 tabs fit; 30 tabs show scroll arrows at both ends, the strip grows up to the window buttons, Ctrl+1 scrolls the first tab into view with the left arrow grayed, Ctrl+Shift+Tab wraps to the last tab, and 29 quick Ctrl+W presses closed cleanly.
- **Tab autosave (#358).** With "Reopen my tabs" on, three tabs were restored after `kill -9` four seconds after the last change, on X11 and again on Wayland.
- **New files (#358).** New > Text Document is now `-rw-r--r--` (0644).
- **Shortcuts window (#361).** Keys read "Ctrl+Shift+N", "Alt+Left arrow", "Esc", "Ctrl+Spacebar", and search ("select") filters the rows.
- **Screen-reader names (#361).** The Details button, search box ("Search Pictures"), tabs (path as description), column titles and rows (no hidden extension) are named as they show.
- **Settings.** Search finds "extensions", "replace", "thumbnails", "dark mode" and "check boxes"; the Replace setting saves as `recycleReplacedFiles`; Theme Dark applies at once.
- **2,000 files.** The folder opens in under a second; all eight views switch at once; Ctrl+A with the Details pane reads "2,000 items selected 47.8 KB"; Delete to the Recycle Bin took 0.5 s and Undo restored all 2,000 in 3 s.
- **Archives (#360).** Report.7z browses into folders, Extract all works and shows the result, ZIP extraction into an existing folder asks "The destination has 3 files with the same names", and the header-encrypted 7z explains itself and offers "Open in archive manager".
- **Copy, move, Undo.** Cut and paste to Downloads and Ctrl+Z moved it back; Copy as path gives a quoted path.
- **Search.** "notes" finds the three notes with highlighted matches, and `kind:document` finds nine items across subfolders.
- **Keyboard.** Tab cycles through the whole window with no trap; F4 opens the address history.

## Counts

| Table | Count |
|---|---|
| Bugs | 14 |
| Unlike Windows (Microsoft-confirmed) | 0 (Alt+D is cited under Bugs) |
| Unlike Windows (VM-confirmed 2026-11-18) | 9 |
| Not sure | 8 |
| Still broken from earlier reviews | 2 (1 partly fixed, 1 withdrawn) |
| Worked well | 17 |
