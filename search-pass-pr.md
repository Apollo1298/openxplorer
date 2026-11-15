# Title

feat(search): Read quoted phrases, AND, OR, NOT, name:, ext:, date ranges and sizes in the search box

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

The folder's own items, the walk of its subfolders, a search of file contents and the search cache all read a search the same way, so an indexed folder and one nobody indexed give the same results. The yellow marks in result names cover the words and phrases searched for, and no longer the operators or the words after `NOT`.

Speed is unchanged: the parser adds no measurable time to a walk of 100,000 files.

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
| 100,000 files, word found nowhere (full walk) | 1,361 ms | 1,376 ms | not measured |
| 100,000 files, `report` (first 500 shown) | 360 ms | 330 ms | not measured |
| 100,000 files, `ext:pdf` | 0 results | 500 results in 442 ms | not measured |
| 10,000 files, `report`, results complete after the last key | 633 ms | 654 ms | not measured |

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

- New tests that fail on the previous main: `native/crates/ox-app/src/window/tests/search_options.rs` (`advanced_query_syntax_finds_files_in_a_folder_nobody_indexed`, `advanced_query_syntax_finds_files_in_an_indexed_folder`, `the_marks_leave_out_operators_and_excluded_words`), plus unit tests in `native/crates/ox-core/src/search/` (`expression.rs`, `pattern.rs`, `query.rs`, `syntax.rs`, `facets.rs`).
- `python3 native/tools/check.py` (rustfmt, Clippy and every test executable, each on its own Xvfb display) and `python3 native/parity/check.py`.
- Real keyboard input with xdotool on fictional files under Xvfb at 1x and 2x, light and dark, normal and Compact view, and under weston 13 (headless, through Xwayland): each search above, opening a folder result and going Back, Close search, Escape, and leaving the folder in the middle of a 100,000-file walk (the address bar answered in 36 ms and no stale results appeared). No GTK critical warnings.
- Pictures taken with `tools/native_capture.py` on its fictional demo tree.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
