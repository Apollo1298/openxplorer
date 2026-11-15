# Overnight review 3: OpenXplorer main dd39e3b

Reviewed upstream main `dd39e3ba2d56e5a51a2e45ef10cb4d6c5e45e5e6` (after #326). This is a report only: no code changes, no branches, no PRs.

**Rank** is how soon someone switching from Windows would notice it: 1 = in the first minutes, 5 = rarely.

**Reproduced** says how each finding was checked:
- "Yes (X11)" or "Yes (Wayland)": driven with real xdotool input on the release build.
- "Code reading only": not run.

**Left out on purpose:**
- Areas other jobs own tonight: network shares, big-folder speed, Open/Save dialogs, Gallery, drag and drop, archives, search, and Recycle Bin/delete.
- Anything the unmerged branches top-bugs, more-bugs, windows-differences-2, network-thumbnails and supere-uninstall already fix: tooltips over menus and dialogs, the window opening larger than the screen, the update error wording, the Preview pane text margin, and the 128 Chinese strings.
- Findings from the first two reviews, unless they are still broken.

## How it was tested

- **Build.** A release build of dd39e3b, run as `tester` with Rust 1.92.0.
- **Isolation.** Never on a live desktop.
  - X11: a private Xvfb display, a private D-Bus session and a throwaway HOME under `/home/tester/runs/<n>`.
  - Wayland: Weston (x11 backend, scale 2) nested in a private Xvfb display.
- **Fictional files.** Documents holds Budget 2026.txt, Notes 1-3.md, plan.pdf, data.json, page.html, script.py and Projects/Sub. Pictures holds sunset.png and circle.jpg. There is also a short tone.ogg and clip.webm, plus a "Weird" folder for the crash tests.
- **Combinations.**
  - 1366×768 at 1x (X11).
  - 2732×1536 at 2x (X11, zh_CN).
  - 3840×2160 at 2x (X11).
  - Weston at 2x, dark theme, Compact view.
  - Light and dark, switched live through `org.gnome.desktop.interface color-scheme`.
  - High contrast through `org.gnome.desktop.a11y.interface high-contrast`.
  - Large text through `Xft.dpi: 192`, and through the app's own maximum text size.
- **Screen-reader names.** Read through AT-SPI (pyatspi under Python 3.12).
- **Installed for this run:** xdotool, gettext, the en_US, de_DE and zh_CN locales, at-spi2-core, and bubblewrap. Bubblewrap is needed for PDF previews.
- **Checks run.**
  - `native/parity/check.py`: passed.
  - `native/tools/i18n.py check`: passed.
  - The cargo test suite and `native/tools/check.py`: not run tonight.
- **Not tested.**
  - The update install flow: GitHub answered 403 through this container's proxy.
  - Real removable media: no udisks here, the kernel can't mount vfat, and GIO hides a tmpfs under /run/media.
  - A long-text locale: there is no de catalog in `native/po`.
  - Orca speaking. Only the AT-SPI names were read.
  - A real logout.
  - Reduced motion at runtime. It was checked by code reading only (GTK drops CSS transitions when animations are off).
- **Environment limits.** Xvfb has no window manager, so dialogs open at the top-left and menus have black corners. After another program (LibreOffice) took the X focus, keys went nowhere until the window was focused again. These are not app bugs and are left out.

## Bugs

| Rank | What happens | Exact steps | Suggested fix | Reproduced |
|---|---|---|---|---|
| 2 | **"Use the desktop font" cannot be turned on.** The switch springs back off, and `desktopFont` never reaches settings.json. The setting therefore never works, so the desktop font's size never applies either. | Settings > Appearance, click the "Use the desktop font" switch. | The switch's own handler starts an asynchronous save (`settings_page/bindings.rs:84`). Its second handler (`settings_page/appearance.rs:351`) redraws the text, and `theme.rs:238-252` then emits text-size-changed. `follow_shared_state` (`bindings.rs:41-60`) answers with `show_current_preferences` (`bindings.rs:110`), which reads the old value and turns the switch off again. Fix: update the in-memory preferences before redrawing, or skip the redraw while the page writes. The bug shows whenever the desktop font differs from the 13 px Windows text, which is the usual case. | Yes (X11 1x and Wayland 2x) |
| 2 | **A mistyped address shows a raw GIO error.** | Alt+D, type `/home/<you>/Documnets`, Enter. A toast reads: "Error when getting information for file “/home/…/Documnets”: No such file or directory". | Say in plain words that the location wasn't found and the spelling should be checked, and name the typed text. Keep the system text out of the main message. | Yes (X11) |
| 3 | **New files are private (mode 0600).** A text file, bitmap, RTF or ZIP made from New is `-rw-------`. Files made by other apps get 0664 or 0644 (umask 0002 here). A New folder is 0775. Other users and the group can't read the new file, which matters in shared folders. | Documents > New > Text Document (also Bitmap image, Rich Text Document, Compressed (zipped) Folder), then `ls -l`. | `ox-core/src/ops/new_from_template.rs:384` creates the stage with `FileCreateFlags::PRIVATE` and never changes the mode after the rename. Set the mode from the umask (0666 & ~umask) before or after publishing the stage. | Yes (X11) |
| 3 | **The Preview pane shows an RTF file as raw markup.** | Alt+P, select "New Rich Text Document.rtf". The pane shows `{\rtf1\ansi\deff0{\fonttbl{\f0 Calibri;}}…`. | `is_text` (`window/details_pane/preview_page.rs:165-171`) accepts `application/rtf` because shared-mime-info makes it a subclass of text/plain. Exclude RTF, and show "No preview available." until there is a renderer. | Yes (X11) |
| 3 | **This PC: the Details pane and status bar ignore the drives.** The Details pane says "This PC (0 items)" with a yellow folder icon, and the status bar says "Ready" even with Local Disk selected or right-clicked. | Click This PC with the Details pane open. Click or right-click Local Disk. | Count and describe the This PC tiles like list items: "1 item", "1 item selected", and the drive's own icon and facts in the pane. | Yes (X11) |
| 3 | **Large text cuts off the file list and navigation pane.** At 200% text, columns and the navigation pane keep their 1x widths. Dates read "11/15/2026 …" and types "Ogg Vo…". The pane shows "Downlo…" and "Docum…". Command bar and pane icons stay at 16 px. The app's own largest size (Ctrl++ 8 times) does the same. | `Xft.dpi: 192` (GNOME Large Text at 2.0 reaches GTK as this), restart, open Music or Documents. | Scale the default column widths and the navigation pane width with the text factor, or fit the columns when the text size changes. | Yes (X11) |
| 3 | **"Reopen my tabs" loses tabs when the app is ended by a signal.** Tabs are saved only when the last window closes normally or on Ctrl+Q. A SIGTERM, as at logout or shutdown, keeps the session from the previous close. Only the last window to close is saved. | Turn on Settings > General > "Reopen my tabs…". Open a Music tab, then `kill -TERM` the app and start it again: the Music tab is gone. Closing with the window's X or Ctrl+Q works. | Save the session when it changes, debounced, or on SIGTERM and GApplication shutdown, not only in `close_request` (`window/imp.rs:447`, `window/session_restore.rs:94`). | Yes (X11; logout itself not tested) |
| 3 | **A Chinese sentence has English in the middle.** It reads "ZIP 打开使用 the desktop choice，打开下载的文件不属于“在文件夹中显示”。" | `LANGUAGE=zh_CN`, Settings > ZIP & archives > "From other apps". | `integration/status.rs:58` builds `"the desktop choice"` without gettext. Mark it for translation, or better, make two whole messages. | Yes (X11 2x) |
| 4 | **A drive's capacity differs between This PC and Properties.** | This PC tile: "27.6 GB free of 252 GB". Local Disk Properties: free "27.5 GB", capacity "251 GB" (270,553,174,016 bytes). | Use one formatter for both: one rounds, the other truncates. | Yes (X11) |
| 4 | **The shortcuts window spells keys differently from the menus and Microsoft.** It shows "Shift+Ctrl+N", "Shift+Alt+P", "Shift+Ctrl+1…8", "Alt+Return" and "Escape". The New menu says "Ctrl+Shift+N", and Microsoft writes Ctrl+Shift+N, Alt+Shift+P and Alt+Enter. | Ctrl+?, read the Files and View groups. | `window/shortcuts_window.rs:235` uses `gtk::accelerator_get_label`. Format the modifiers in Windows order (Ctrl, Alt, Shift) and use "Enter" and "Esc", as the menus do. | Yes (X11; read through AT-SPI) |
| 4 | **The shortcuts window leaves out working keys that Microsoft lists.** Missing: Backspace (back; it works), Home and End, Shift+arrows, Ctrl+arrow then Space, and Left and Right in the navigation tree. | Ctrl+?, search for "Backspace" or "Home". | Add a group of fixed rows for the list and tree keys. | Yes (X11) |
| 4 | **"New Bitmap image" has no visible icon in the light theme.** The new BMP is a 1×1 white picture, and its thumbnail is drawn with no frame, so the row looks iconless. In dark it is a white square. | New > Bitmap image, Details view. | Give thumbnails a 1 px frame, or use the type icon for pictures smaller than the icon. | Yes (X11) |
| 4 | **Settings search misses common words, and one row looks like a setting with no control.** "extensions" and "language" find nothing. "hidden files" finds "How sizes are counted" and "Indexed folders". "Calculate folder sizes" on Files & folders has a title and an info button, but nothing to change. | Type in "Search settings". Open Files & folders. | Add search keywords that point to View > Show and Folder Options (file name extensions, hidden items). Turn the "Calculate folder sizes" row into help text, or merge it into "How sizes are counted" (`settings_page/files_folders.rs:411`). | Yes (X11) |
| 4 | **Screen-reader names.** | Read through AT-SPI in Documents. | Fix each on its own, as listed after this table. | Yes (AT-SPI) |

Notes on the screen-reader names row:
- The command bar's Details pane button has an empty name.
- The search box is named "Search filenames and paths", with the description "Search cached filenames and paths. Enable folders in Settings → Search.", while it shows "Search Documents".
- Every tab's description is "Close tab".
- The column-header row repeats its description twice.
- Rows are named with the extension ("Budget 2026.txt") while the list shows "Budget 2026".

## Unlike Windows (Microsoft-confirmed)

Microsoft's list is the Windows 11 File Explorer table at https://support.microsoft.com/en-us/windows/keyboard-shortcuts-in-windows-dcc61a57-8ff0-cffe-9796-cb9706c75eec. It was compared key by key with the app's Ctrl+? window and with real input.

**No new differences found.** These File Explorer keys from Microsoft's list were pressed tonight and behave as described:
- Alt+D, Ctrl+T, Ctrl+W and Ctrl+1.
- Alt+P, Alt+Shift+P and Alt+Enter.
- Alt+Left and Backspace.
- F2, F5, Ctrl+Shift+N, Ctrl+Z and Ctrl+Y.

The rest of Microsoft's table was compared only with the Ctrl+? window (Bugs, last shortcut rows) or covered by the earlier reviews.

The one remaining difference is from an earlier review and is listed under "Still broken".

## Not sure (needs a Windows check)

| Rank | What OpenXplorer does | What to look at on Windows 11 |
|---|---|---|
| 3 | With 19 tabs at 1366 px, every tab label collapses to "…", even "Home". There are no overflow arrows, but the mouse wheel over the strip scrolls it. | How Windows shows many tabs: shortened titles, and scroll buttons? |
| 3 | Folder Options > General: "Single-click to open an item (point to select)" and its two underline choices are grayed out. | Windows offers single-click. Decide whether to implement it or hide it. |
| 3 | Settings > Files & folders: "When replacing a file, move the old one to the Recycle Bin" is on by default, so Replace keeps the old file in the bin. | Does Windows' Replace put the old file in the Recycle Bin? |
| 4 | View > Show opens only on a click, not on hover. | Windows opens submenus on hover. |
| 4 | Security > Edit, Allow "Full control" on a .txt gives it `rwx`, so it becomes executable. | Not a Windows question: should Full control set the execute bit on plain files? |
| 4 | Properties > General name box shows "Budget 2026.txt" with extensions hidden. | Does Windows show the extension there? |
| 4 | Opening Home, or any folder, adds the folder to the desktop-wide `recently-used.xbel`, so other apps' Recent lists fill with folders. | Windows' Recent shows files; folders go to Quick access. Decide what to log. |
| 4 | zh_CN: the list shows "Documents" while the navigation pane says 文档. Dates read "2026年11月15日 04时19分". | Windows localizes known folder names everywhere. Check the zh-CN short date format. |
| 4 | The File Conflict window says "1 bytes". | Windows' wording for a 1-byte file. |
| 5 | Settings > Default apps: the three "When making OpenXplorer default" switches aren't saved, and are back to their defaults the next time Settings opens. | Probably by design (one-shot options). Confirm. |
| 5 | Without bubblewrap the Preview pane says "No preview available." for every PDF, with no hint why. The packages don't list bubblewrap or poppler-utils as a dependency or recommendation (`native/packaging`). | Not a Windows question: add a Recommends, or name the missing tool in the pane. |

## Still broken from earlier reviews

| Rank | Finding (review) | State on dd39e3b | Reproduced |
|---|---|---|---|
| 4 | Compact view sits under View > Show, not in the View menu itself (review 1, Clearly wrong). Microsoft: the View menu "includes Compact view" (https://support.microsoft.com/en-us/windows/experience/fileexplorer/file-explorer-in-windows). | **Still broken.** The View menu lists the eight views, the panes, Show, Split view, Folder tree, Terminal and the text sizes. Compact view is only in Show. | Yes (X11) |
| 3 | About a quarter of the Chinese UI is English (review 2). | **Still broken.** Against the current .pot, 295 strings are untranslated and 337 fuzzy (632 shown in English). more-bugs adds 128, so roughly 500 stay. Settings still shows these in English: "Open Quick access on Home", "Show more items in the navigation pane", "Copying and moving", "Search results", "Show the search filter bar", "Check for updates automatically", "Show the Update button in the command bar", "Show keyboard shortcuts" and "Open the manual". Nothing was cut off at 2x. | Yes (X11 2x) |
| 4 | Split view at 1x cuts off columns (review 2). | **Still broken.** With the Details pane open at 1366 px, each pane shows only Name and Date modified, with no scroll bar. more-bugs left this unchanged on purpose. | Yes (X11) |

## Worked well

- **Settings.**
  - All 39 switches on the eight pages were toggled on and off through AT-SPI. 32 saved the expected key to settings.json and removed it again.
  - Not counted in those 32:
    - "Use the desktop font" (Bugs).
    - Three Search index switches, which store elsewhere and were not followed further.
    - The three Default apps options (Not sure).
  - Search shows "N matching settings", grouped by page, and "No settings match your search.".
  - Theme Light/Dark/System and Compact view apply at once, on X11 and Wayland.
  - Relative dates ("Today at 4:11 AM") and "Show the number of items in folders" ("3 items") work.
- **Folder Options.**
  - View > Show hidden files, Hide extensions and Show status bar apply live.
  - Restore Defaults puts them back.
  - Folder Options and Settings share one store.
- **Properties.**
  - The General tab gives size, size on disk and dates.
  - The Read-only and Hidden attributes apply.
  - Security > Edit changes `chmod` correctly.
  - The Details and Previous Versions tabs work.
  - Folder Properties and multiple-item Properties ("8 Files, 1 Folders", "Contains: 8 Files, 2 Folders") work.
  - Local Disk Properties shows its ring.
- **Rename.**
  - The invalid-character balloon matches Windows' text and refuses the character.
  - Esc cancels.
  - Tab and Shift+Tab move to the next and previous item's rename.
  - The extension is kept.
- **Undo/Redo.**
  - A chain of two renames plus a Properties Hidden rename undoes in order.
  - Ctrl+Y and Ctrl+Shift+Z redo.
  - Undo of "keep both" copies removes the copies.
  - Undo of a cut-paste that replaced a file brings back both the moved file and the replaced one.
- **New menu.** Folder, Shortcut, Bitmap image, Rich Text Document, Text Document and Compressed (zipped) Folder are all created with Windows' names, and the second one gets "(2)".
- **Local conflicts.**
  - "The destination has 2 files with the same names" offers Replace, Skip and Let me decide.
  - The File Conflict window's "both versions" gives "Notes 3 (2).md" and "data (2).json".
- **Preview pane.** Pictures, text, JSON, HTML, PDF (with bubblewrap), video (frame with a play button) and audio (player) all preview.
- **Details pane.** A single picture shows its dimensions, and several items show "2 items selected" with the total size.
- **Tabs and windows.**
  - Restore on restart (tabs, order, history) works after a normal close or Ctrl+Q.
  - Dragging a tab out makes a new window.
  - The mouse wheel scrolls an overflowing tab strip.
- **Accessibility.** High contrast and dark mode follow the desktop live.
- **Crash resistance.** No crashes, freezes or Gtk-CRITICALs in any session. This covered:
  - Names with a leading or trailing space, `$(id)`, `-rf`, a backslash, `q?"<>|*`, a tab, a newline, invalid UTF-8 (shown as "(invalid encoding)"), CON and `x.`.
  - Emoji names, a 60-level tree, a symlink loop, a link to ".", a dangling link, a FIFO, a socket and a chmod 000 folder ("Location is not available … Access is denied.").
  - Browsing /proc, /dev and /sys.
- **Layouts.** 1366×768 at 1x, 3840×2160 at 2x and Weston at 2x lay out correctly, apart from the items above.

## Counts

| Table | Count |
|---|---|
| Bugs | 14 |
| Unlike Windows (Microsoft-confirmed) | 0 |
| Not sure | 11 |
| Still broken from earlier reviews | 3 |
| Worked well | 13 |

No security findings.
