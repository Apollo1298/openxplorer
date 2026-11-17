# Title

feat(ui): Overwrite on Replace, leave hidden extensions out of Properties and scroll overflowing tabs with arrows, as Windows 11 does

# Body

Model: (to be filled in when the pull request is opened)

Three places where OpenXplorer behaved differently from Windows 11 File Explorer (checked in Windows 11, build 26200) now behave as Windows does.

- **Replace overwrites the old file.** In "Replace or Skip Files", Replace no longer moves the old file to the Recycle Bin by default. Settings > Files & folders > "When replacing a file, move the old one to the Recycle Bin" is kept for anyone who wants it, but it is now off. The overwrite stays safe: the new file is written under a private temporary name and swapped in at the end, so a copy that fails (a full disk, a read-only folder) or is canceled leaves the old file as it was. Undo after an overwriting Replace used to be silently unavailable, so Ctrl+Z reached past the Replace to an older operation. Now Undo says plainly, in a Windows message box, that it can't bring the old file back ("“report-a.txt” was replaced, so Undo can't bring back the old file."). When the same copy also brought in new files, it asks "Undo the rest anyway?" and then takes back only those.
- **Properties leaves out a hidden extension.** With "Hide extensions for known file types" on (the default), the General tab's name box shows `extra_3` for `extra_3.txt`, and the window is titled "extra_3 Properties". A name typed there keeps the hidden extension, as renaming in place does: typing `harbor notes` for `notes.txt` gives `harbor notes.txt`. Folders, names without a known type's extension and dotfiles show whole, and with extensions shown nothing changes. The Properties window keeps its fixed size.
- **Many tabs scroll with arrows.** Tabs shrink only as far as still shows the icon, about three letters of the title and the close button. Past that, the strip scrolls, with an arrow at each end that scrolls it one tab per click, shown only while the tabs overflow. The "+" stays right after the strip, the mouse wheel still scrolls it, and the tab brought to the front (a click, Ctrl+Tab, a new tab) scrolls into view. Closing tabs until they fit hides the arrows again.

| | Before | After | Windows 11 |
|---|---|---|---|
| Replace, old file | Moved to the Recycle Bin (setting on by default) | Overwritten; the setting is off by default and still works | Overwritten; nothing new in the Recycle Bin |
| Undo after an overwriting Replace | Not offered; Ctrl+Z undid the operation before it | Says it can't bring the old file back; undoes the rest of the copy if asked | Not confirmed |
| Properties name box, extensions hidden | `extra_3.txt` | `extra_3` | `extra_3` |
| Properties title, extensions hidden | "extra_3.txt Properties" | "extra_3 Properties" | "extra_3 Properties" |
| Name typed in Properties, extensions hidden | Taken as the whole name | The hidden extension is kept | Not confirmed beyond the box and title |
| 25 tabs in a 1920-pixel window | Titles shrink to "…", no arrows | Icon, about three letters and the close button; ◀ ▶ arrows at the ends | Icon, about three letters and the close button; ◀ ▶ arrows at the ends |

## Changed owner decisions

- OPS-028: Replace overwrites by default. "When replacing a file, move the old one to the Recycle Bin" is now off by default and stored as `recycleReplacedFiles` only when on. A settings file that saved it off keeps it off; an installation that never touched it changes from recycling to overwriting.
- PROP-005: the Properties name box and title follow "Hide extensions for known file types".
- TAB-019: at their narrowest, tabs keep room for five average characters of their title (about three letters and the ellipsis), and an overflowing strip shows scroll arrows at both ends.

## Not confirmed in Windows

- What Windows' Undo does after a Replace (this change says it can't bring the old file back).
- Whether Windows' tab scroll arrows take keyboard focus (here Tab skips them; the arrow keys, Ctrl+Tab and the wheel reach every tab), their exact glyph (chevrons here) and how far one click scrolls (one tab here).
- The title of several items' Properties and of the Security tab's Permissions window while extensions are hidden (both still give whole names here).
- Whether a launcher's (`.desktop`) Properties title should drop its extension even with extensions shown, as Windows always hides `.lnk`.

## How it was tested

Each change has a test that fails on the unchanged main branch and passes here: the settings default, Replace overwriting by default with Undo's notice (one file, and several files with a new one), the core default, Properties with extensions hidden (name box, title, rename keeping the extension, folders and names without an extension whole), and the tab strip at 1366 and 1440 pixels with 5, 20 and 40 tabs (arrows only while the tabs overflow, the "+" in the window, about three letters kept, the arrows scrolling the strip, Ctrl+Tab bringing an off-screen tab into view, the arrows gone after closing tabs). A guard test checks that nothing changes with extensions shown. Existing tests that relied on the old Replace default now turn the setting on, and Properties titles in tests follow the hidden extension.

By hand, with real keyboard and mouse input (xdotool) on a private X display, fictional files, no crashes and no GTK critical warnings:

- Replace: one file and several files with the setting off and on; Undo after each; a read-only destination folder, a full disk and a disk out of inodes (so the write itself fails) each leave the old file intact with nothing left behind. At 1x light and at 2x dark in Compact view.
- Properties: a file, a file named `extra_3.txt`, a folder, a file without an extension, a dotfile and several items, with extensions hidden and shown; renaming from the name box; the window stays 388 x 528. At 1x light and at 2x dark in Compact view.
- Tabs: 5, 20 and 40 tabs at 1366 and 1920 pixels, 1x and 2x, light and dark, normal and Compact view; clicking the arrows, the mouse wheel, Ctrl+Tab to an off-screen tab, closing tabs until the arrows go, and dragging a tab within the overflowing strip.

The new window and settings tests also pass on a headless Wayland compositor (weston). Real input could not be sent there: the test machine has no Xwayland.

Checks: `cargo fmt --check`, Clippy with the workspace lints and `-D warnings`, `python3 native/parity/check.py`, the message template check, the docs sync check and the public-data audit pass. `python3 native/tools/check.py` stops at `location_fixture_drift`, which fails on the unchanged main branch too, so every test binary was run to the end with the same isolation: 4,790 passed and 33 failed here, against 4,784 passed and 34 failed on the unchanged main branch. Every failure here also fails on main (tests that need the old Python app extracted, and two sandboxed thumbnailer tests).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
