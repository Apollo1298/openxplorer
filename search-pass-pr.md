# Title

feat(search): Read Windows search syntax in the search box and spin a progress ring on the tab while a search runs

# Body

Model: claude-opus-5-5

## Summary

The search box now understands Windows Search's Advanced Query Syntax as Microsoft documents it. Before, every one of these searches found nothing, because the quotes, the operators and the property names were searched for as plain text:

- A phrase in quotes is found as typed: `"annual report"`. A doubled quote stands for a quote, and quoted text is never syntax.
- `AND`, `OR` and `NOT` in capitals combine words (`report NOT draft`, `budget OR invoice`). Lower-case `and`, `or` and `not` stay words. Words apart are all required, the operators bind from left to right, and parentheses group (`annual (budget OR 2026)`).
- `name:`, `filename:` or `file:` look in the name alone, with `=`, `~<` (begins with), `~>` (ends with), `~=` (contains), `~!` (does not contain), `~` (wildcards for the whole name) and `<>` (not equal).
- `ext:` or `fileext:` choose the extension, with or without its dot: `ext:pdf`, `ext:.pdf`, `ext:(pdf OR docx)`, `ext:<>pdf`.
- `date:` reads as `datemodified:`. A date can be compared or ranged: `date:>2026-01-31`, `date:<=1/31/2026`, `date:11/13/26..11/15/26` (both days included). Dates are ISO 8601 or month/day/year.
- Sizes take `kb`, `mb` or `gb` (1,024 bytes a kilobyte), compared or ranged: `size:>1MB`, `size:5kb..10kb`, `size:=0`.

While a search's results are still coming in, the tab in front now shows a small spinning progress ring in the accent color in place of the magnifier, as Windows 11 File Explorer does (build 26200), and the magnifier once the search finishes or is canceled. A tab opening or listing a folder, in front or behind, shows the same ring until the folder is listed. With the desktop's animations off the ring stands still. The tab reports itself as busy to screen readers while the ring shows. The ring is drawn the way the drive ring in Properties is: its pictures are computed at the screen's scale and shown in turn, with no icon drawn in code.

The folder's own items, the walk of its subfolders, a search of file contents and the search cache all read a search the same way, so an indexed folder and one nobody indexed give the same results. The yellow marks in result names cover the words and phrases searched for, and no longer the operators or the words after `NOT`.

The parser adds no measurable time: a walk of 100,000 files takes as long as before. In a folder of 10,000 files the first key typed still holds the window for about a quarter of a second while GTK builds the result rows and shows the Folder path column; that is unchanged and not addressed here.

## Before, after and Windows

| Search | Before | After | Windows (Microsoft's documentation) |
|---|---|---|---|
| `"annual report"` | 0 results | the file whose name holds the phrase | "Finds items that contain the exact phrase" |
| `report NOT draft` | 0 results | reports without "draft" | `NOT` in capitals excludes |
| `budget OR invoice` | 0 results | either word | `OR` in capitals |
| `report and annual` | also needs "and" in the name | same: lower-case `and` is a word | operators must be upper case |
| `ext:pdf` | 0 results | PDF files | `ext:.txt` |
| `name:draft` | 0 results | names holding "draft" | `filename:MyResume` |
| `date:today`, `date:>2026-01-31`, `date:a..b` | 0 results | items modified in that range | `date:>11/13/21`, `date:11/13/21..11/15/21` |
| `size:>1MB`, `size:5kb..10kb` | 0 results | files in that range | `System.Size:5kb..10kb` |
| Tab icon while a search runs or a folder lists | magnifier or folder icon throughout | spinning ring, then the magnifier or folder icon | spinning blue ring, then the magnifier (build 26200) |
| 100,000 files, word found nowhere (full walk) | 1,361 ms | 1,376 ms | not measured |
| 100,000 files, `report` (first 500 shown) | 360 ms | 330 ms | not measured |
| 100,000 files, `ext:pdf` | 0 results after a full walk of 1,435 to 1,491 ms | 500 results in 452 to 494 ms | not measured |
| 10,000 files, `report`, results complete after the last key | 598 to 646 ms | 667 to 821 ms (681 to 720 ms with the listing changes this branch includes and none of the search changes) | not measured |
| 10,000 files, first key typed into the search box: the box redraws after | 260 ms | 246 to 272 ms | not measured |

Timings are from a release build driven with xdotool under Xvfb at 1x, from the last key to the last change of the status bar.

Sources: <https://learn.microsoft.com/en-us/windows/win32/search/-search-3x-advancedquerysyntax> and <https://learn.microsoft.com/en-us/windows/win32/lwef/-search-2x-wds-aqsreference>.

## CHANGED OWNER DECISIONS

- SRCH-044 said any other `name:value` is a word to search for. Now `name:`, `filename:`, `file:`, `ext:`, `fileext:` and `date:` are read as properties, as Microsoft documents them. Other `name:value` pairs (`author:`, `folder:`) are still words.
- A search option (`kind:`, `date:`, `size:`) after `NOT`, next to `OR` or in quotes stays a word, because the options narrow every result and cannot be left out or offered as one choice.

## NOT CONFIRMED

- `-word` as "not": Microsoft documents `-` only as a property's not-equal (`System.Kind:-picture`), so a plain `-draft` is still a word.
- `MB` and `GB` units, and whether a kilobyte is 1,000 or 1,024 bytes: the documentation shows only `kb`.
- Dates written day first, and which date format Windows reads in each locale ("date formats ... vary among locales").
- The Search options menu's Other properties and Advanced options (System files, Zipped folders), and a recent-searches list under the search box: no Microsoft page found documents them.
- Where Windows' Alt+Up goes while the search box has the keyboard.
- How long Windows takes to show its first result in a large folder.

## How it was tested

- New tests that fail on the previous main: `native/crates/ox-app/src/window/tests/search_layout.rs` (`the_tab_shows_a_progress_ring_while_the_search_runs`, `a_canceled_search_takes_the_ring_away`, `a_tab_listing_a_folder_shows_the_ring_until_it_is_listed`), plus `the_ring_stands_still_while_animations_are_off` and the ring's unit tests in `native/crates/ox-app/src/progress_ring.rs`; `native/crates/ox-app/src/window/tests/search_options.rs` (`advanced_query_syntax_finds_files_in_a_folder_nobody_indexed`, `advanced_query_syntax_finds_files_in_an_indexed_folder`, `the_marks_leave_out_operators_and_excluded_words`), plus unit tests in `native/crates/ox-core/src/search/` (`expression.rs`, `pattern.rs`, `query.rs`, `syntax.rs`, `facets.rs`).
- `python3 native/parity/check.py`, rustfmt and Clippy with the workspace lints pass. Every test executable and the doctests ran as `native/tools/check.py` runs them (own Xvfb display, private D-Bus, disposable home, the Python app's sources extracted): 4,690 passed and 11 failed, all eleven the same way on the previous main in this environment (nine `admin_copy` tests and two sandboxed-thumbnailer tests). `location_fixture_drift` needs Python 3.12; Python 3.13.18 parses `smb://[V1.x]/` differently. `window::tests::tooltips::no_tooltip_stays_over_a_menu_the_rename_box_or_the_next_folder` passed in the last run but failed about one run in eight earlier; it is not about search.
- Real keyboard input with xdotool on fictional files under Xvfb at 1x and 2x, light and dark, normal and Compact view, and under weston 13 (headless, through Xwayland): each search above, opening a folder result and going Back, Close search, Escape, the ring turning on the tab during a 100,000-file walk (the tab's pixels change between pictures 120 ms apart, at 1x and 2x, light and dark, and under weston) and the magnifier after it, and leaving the folder in the middle of a 100,000-file walk (the address bar answered in 36 ms and no stale results appeared). No GTK critical warnings.
- Pictures taken with `tools/native_capture.py` on its fictional demo tree.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
