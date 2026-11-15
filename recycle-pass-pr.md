# Title

fix(recycle-bin): Empty a large Recycle Bin in seconds and open its items as Windows does

# Body

Model: claude-opus-5-5

Emptying a Recycle Bin of 50,000 items took more than four minutes, and after recycling that many items the Recycle Bin could look empty for two minutes. Both now take seconds. Opening an item in the Recycle Bin shows its Properties instead of launching it, Properties works there at all, and the Recycle Bin shows Date modified beside its own columns.

What changed:

- **Empty Recycle Bin and Delete in the Recycle Bin are fast.** Items that the Trash backend reports in a Trash folder the freedesktop Trash specification names (the home Trash, a drive's `.Trash-$UID` or `.Trash/$UID`) are deleted from disk: the item first, through the existing pinned permanent delete, then its `.trashinfo`. Anything else still goes through GVfs. Folders inside an item are made owner-only first, as GVfs does, so read-only folders go too. An item deleted only in part keeps its `.trashinfo` and stays listed.
- **No backlog after big operations.** GVfs keeps a watched Trash folder's record in a sorted list and walks it from the start for every added or removed name. Items are now deleted in ascending and recycled in descending byte order of their names, so each lookup ends at once. Every Recycle Bin listing also shares one process-wide watch of `trash:///` instead of subscribing its own; GVfs kept sending changes to those after they were dropped.
- **The delete questions are Windows message boxes.** Delete, Shift+Delete and Empty Recycle Bin ask in the shared message box that Rename and Confirm Save As use: the caption ("Delete File", "Delete Folder" or "Delete Multiple Items") in the title bar, the warning icon beside the question, one item's details under it, and Yes, which has focus, and No, also pressed with Alt+Y, Alt+N, Y and N. The wording and counts are unchanged.
- **Progress.** Delete and Empty in the Recycle Bin now count the items done in the progress window (at most one report per 80 ms). Before, they showed no progress.
- **Undo of a large Move to Trash** no longer scans the whole Recycle Bin once per item.
- **Opening an item in the Recycle Bin** (double-click or Enter) shows its Properties, as Windows does, instead of opening the file in its app or browsing into the folder. Properties from the item's menu works now; before, it said the address was not supported. The General tab names its Origin and the date it was Deleted, and a trashed folder's Size and Contains are counted.
- **Columns.** The Recycle Bin lists Date modified after Original location and Date deleted. A new view also puts those two right after Name; before, it kept them in the order they were made until the columns changed. In a search inside the Recycle Bin, Folder path shows where each item came from instead of "/".

| Case | Before | After | Windows 11 |
|---|---|---|---|
| Empty Recycle Bin, 50,000 items, clicked in the window | 265 s | 19 s, listed right 13 s later | Fast (not measured here) |
| Empty Recycle Bin, 6,000 items, watched (test) | 22 s | 2 to 4 s | |
| Empty Recycle Bin, 10,000 items | 15 to 20 s, no progress | about 3 s, progress window counts the items | Progress window |
| Recycle Bin listing after recycling 30,000 items | 26 s (at 50,000: empty for about 2 minutes) | 4 s | Listed at once |
| Undo of recycling 10,000 items | 50 s | 15 s | |
| Double-click a file in the Recycle Bin | Opens it in its app | Properties with Origin and Deleted | Properties |
| Properties from a Recycle Bin item's menu | "Only local paths… are supported" | Properties | Properties |
| Recycle Bin columns | Name, Original location, Date deleted, Type, Size | Name, Original location, Date deleted, Date modified, Type, Size | Name, Original Location, Date Deleted, Size, Item type, Date Modified |
| Delete, Shift+Delete and Empty Recycle Bin questions | Large dialog with the caption as a bold heading | Message box: caption title, icon, question, Yes/No | Message box with caption, icon, question, Yes/No |
| Folder path of a search result in the Recycle Bin | "/" | The folder it was deleted from | |

## CHANGED OWNER DECISIONS

- VIEW-062: Date deleted no longer replaces Date modified in the Recycle Bin. Date modified is listed after the Recycle Bin's own columns, as Windows 11 shows Date Modified there. In the Recycle Bin, Date deleted's width is the one saved under the width they share.
- OPS-040: an item directly in the Recycle Bin is no longer opened or browsed into. Opening it shows its Properties.

## NOT CONFIRMED

These are not covered by Microsoft's documentation, so they still need checking on Windows 11:

- Whether Windows opens Properties for a folder in the Recycle Bin, as it does for a file, and what several selected items open. Here both show Properties.
- The exact rows of a Recycle Bin item's Properties in Windows 11 ("Origin", "Deleted").
- The icon of Windows' delete questions; the app uses the warning icon of its other message boxes, with one item's details under the question.
- The column order: Windows shows Size before Item type and Date Modified last. Here the folder's chosen order is kept after the Recycle Bin's own columns.
- What Windows says when an item cannot be recycled because its name is too long for the Trash (here: "Interrupted Action", with the item kept), and whether it offers to delete it permanently instead.
- Whether Windows disables Delete on read-only media. Here Delete asks the permanent-delete question and then reports "Read-only file system", and the file is kept.
- Whether "Display delete confirmation dialog" off also skips the permanent-delete question on drives without a Recycle Bin. Here the separate "Ask before deleting permanently" setting decides that.
- Whether Windows offers Delete for rows of Recent files on Home. Here it stays off, as the app decided earlier.
- Recycle Bin properties (maximum size, "Don't move files to the Recycle Bin") are not in the app (OPS-044).

Microsoft's documentation used: [Restore deleted files or folders](https://support.microsoft.com/en-us/office/restore-deleted-files-or-folders-in-onedrive-949ada80-0026-4db3-a953-c99083e6a84f) (select the items in the Recycle Bin, right-click, Restore, and they go back to their original folder).

## How it was tested

- New tests that fail on unchanged main: `emptying_a_large_recycle_bin_takes_seconds` (22 s on main, under the 15 s bound now), `opening_an_item_in_the_recycle_bin_shows_its_properties`, `properties_of_an_item_in_the_recycle_bin_shows_where_it_came_from`, `a_new_view_puts_the_listings_own_columns_after_name`, and the updated `the_recycle_bin_shows_and_sorts_by_original_location_and_date_deleted`.
- New tests of the deletion paths: `a_trashed_folder_with_read_only_folders_inside_is_emptied`, `an_item_emptying_cannot_delete_whole_stays_in_the_recycle_bin`, `emptying_the_recycle_bin_reports_how_many_items_are_done`, `items_with_names_that_are_not_utf8_or_long_are_deleted_exactly`, `deleting_one_of_two_items_with_the_same_name_leaves_the_other`, plus unit tests of where items are kept, the deletion and recycling orders, and the shared watch.
- `cargo fmt --check`, Clippy with the workspace lints, `native/parity/check.py`, `native/tools/i18n.py check` and `tools/sync-docs.py --check`: pass.
- Every test executable and the doctests, isolated as `native/tools/check.py` runs them, on this branch and on unchanged main; failures were compared by test name (see below).
- In the app under Xvfb, with real keys and clicks (xdotool) and fictional files: Delete and Ctrl+Z of 8 items; Shift+Delete; Delete in the Recycle Bin; Empty Recycle Bin of 10,000 and 50,000 items, timed; restore onto a taken name (Replace or Skip Files); search and the Details pane in the Recycle Bin; Delete on tmpfs and on a read-only ext4 image; a 254-byte name; and a drive's `.Trash-UID` on a loop-mounted ext4 image (restore and delete). Under headless weston the Recycle Bin view renders with no warnings.
- Pictures at 1x and 2x, light and dark, in Details, List and Compact view, taken with `tools/native_capture.py` and the fictional demo tree.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
