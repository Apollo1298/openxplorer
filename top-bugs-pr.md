# Title

fix(ui): Search from Home and fix hidden selection, rename, tooltip, search title, Home focus and window size bugs

# Body

Model: claude-opus-5-5

This fixes seven bugs that a Windows user runs into in the first minutes.

- **Search from Home.** With "Open Quick access on Home" on, the search box on the Home page was grayed out, so clicking it, Ctrl+E, F3 and typing did nothing. Windows says "To find relevant files from your PC and the cloud, search from File Explorer Home." The box now reads "Search Home" and searches the home folder and its subfolders, with the usual results view. The tab reads "<words> - Search Results in Home", and the status bar counts the results. Back and Close search return to Home. A folder on this computer whose subfolders are only partly in the search cache, as the home folder is once folders are pinned, is now walked whole, its own matches first. Before, the subfolders nobody cached were not searched at all, so neither Home nor the home folder found a file three levels down in an uncached folder. A folder on the network keeps its listing's matches and the cached ones below it.
- **Ctrl+A on Home selected files you couldn't see.** With "Open Quick access on Home" on, Ctrl+A on the Home page selected every item of the folder shown before, which the page hides. Cut, Copy and Delete then acted on them, so Delete could recycle files the user never saw. A page the window draws itself (Home, This PC, Settings) now keeps no rows of the folder the tab left, and Ctrl+A there selects only This PC's own tiles. Home's tiles and rows are selected one at a time, so Ctrl+A selects nothing there. With the option off, Home is the home folder, and Ctrl+A selects what it lists.
- **Ctrl+A while renaming.** Ctrl+A in the rename box selected every item in the folder, and Escape left them selected, so a following Delete would recycle the whole folder. The list or grid around the box took the key for its own Select all. The rename box now keeps Ctrl+A and selects its own text only. Outside the box, Ctrl+A still selects every item.
- **Tooltips over menus, the rename box and the next folder.** GTK 4 keeps a tooltip on screen until the pointer moves, so a file's tooltip covered the menu that Shift+F10, the Menu key or a right-click opened. A column title's tooltip covered its menu, the hovered row's tooltip covered the rename box, and a folder's "Date created" tooltip stayed over the folder that a double-click opened. Now any key or button pressed in the window hides the tooltip, and so does every change of location. No new tooltip shows until the pointer moves. Tooltips also stay off while a menu, another popover or the rename box has the keyboard.
- **Search title.** A tab kept the first search's words ("a?b - Search Results in Documents") after the user typed on to a new search. The tab and the window title now follow every new search.
- **Keyboard focus after a Home menu.** After a Home tile's or row's menu closed, the arrows and Enter did nothing. The keyboard now goes back to the tile or row, whether the menu closed with Escape, by choosing an item that stays on Home, or by a click away.
- **Window larger than the screen.** On Wayland, GTK fits a new window to the whole monitor but not around the panels. At 200 % on a 1650 × 1050 screen (825 × 525 logical pixels), the window ran off the right and bottom edges. A new window's size, saved or default, is now fitted to the monitor in logical pixels, with room left for panels on Wayland. A saved size that fits is kept, and the user's saved size is not overwritten by the fitted one. On X11, GTK already fits windows to the work area.

| Fix | Before | After |
|---|---|---|
| Search from Home | Search box grayed out on Home; click, Ctrl+E, F3 and typing did nothing | "Search Home" searches the home folder and subfolders; "zebra - Search Results in Home", "1 result"; Back and Close search return to Home |
| Partly cached folders | With pinned folders cached, a search of the home folder skipped every uncached subfolder ("No matching items" for files in OX-Test/Deb32) | The whole tree is walked; the files three levels down are found, from Home and from the home folder |
| Ctrl+A on Home | "15 items selected" of the hidden previous folder; Cut, Copy and Delete active | Nothing selected, Cut, Copy and Delete disabled; Delete recycles nothing; This PC selects only its own tiles |
| Ctrl+A while renaming | Every item in the folder selected ("15 items selected"), still selected after Escape | Only the name's text is selected; "1 item selected" before and after Escape; Ctrl+A outside the box selects all |
| Tooltips | A row's or column title's tooltip covered its menu; the hovered row's tooltip covered the rename box; a folder's tooltip stayed over the next folder | Hidden on any key or click and on every change of location; none shows until the pointer moves; off while a menu or the rename box has the keyboard |
| Search title | Tab and window title kept the first search's words | Each new search's words, also after Back, Forward and in a split pane |
| Home menu focus | After Right, Shift+F10, Escape on a tile, Left and Enter did nothing | Left and Enter work after Escape, after choosing an item and after a click away |
| Window size | 1320 × 810 window on an 825 × 525 logical screen ran off the right and bottom edges | Fits the screen below the panel; a saved size that fits is kept; maximize and restore work |

## How it was tested

**Tests first.** Each fix has a test that failed on unchanged `main` and passes now:

- `searching_from_home_searches_the_home_folder_and_its_subfolders`: the box was disabled.
- `searching_from_home_finds_files_deep_in_uncached_folders`: with a cached Documents under home, nothing from an uncached folder three levels down was found. `a_folder_with_an_indexed_subfolder_shows_both` now expects the uncached subfolder's match too.
- `ctrl_a_on_a_page_never_selects_the_hidden_folder`: on Home, all four items of the folder shown before were selected. The companion test `settings_and_closed_search_results_keep_no_hidden_selection` covers Settings and search results after Close search, which already kept no hidden selection.
- `ctrl_a_while_renaming_selects_the_name_not_the_items`: real keys; all items were selected.
- `no_tooltip_stays_over_a_menu_the_rename_box_or_the_next_folder`: real pointer and keys; the tooltip stayed over the Shift+F10 menu.
- `every_new_search_names_its_own_words_in_the_tab_and_title`: the tab kept "notes" after typing on to "notes 1".
- `the_keyboard_comes_back_to_the_item_whose_menu_closed`: focus did not return.
- `a_saved_size_larger_than_the_screen_opens_fitted_to_it`: under Weston at 825 × 525 and 1366 × 768. On X11, GTK already fits the window. On `main` under Weston, the fitted size overwrote the saved one.
- `a_window_fits_small_and_scaled_screens`: a new unit test.

**Checks.**

- `cargo fmt --check` and Clippy with the workspace lints pass.
- `python3 native/parity/check.py` passes. New features HOME-017 (search from Home) and VIEW-073 (no tooltip left over menus, the rename box or the next folder). The notes of SRCH-046, HOME-016, TAB-054, OPS-010 and SEL-004 are updated.
- `python3 native/tools/check.py` ran its inventory, driver, icon, rustfmt and Clippy stages. It then stopped at `location_fixture_drift`, which cannot import Python's `gi` module on this machine.
- Every test executable and the doctests were then run with check.py's own isolation:
  - All 2,354 ox-app tests pass except the two sandboxed-thumbnailer tests, which also fail on `main`.
  - The Python fixture comparisons `location_fixture_drift` and `network_python` fail with the same `gi` import error.
  - ox-core's nine `admin_copy` tests fail in this environment. This change does not touch ox-core.
  - Everything else passes.

**Real input.** The app was driven with xdotool on a private Xvfb display (X11) and in Weston (Wayland), with fictional files only:

- 1x and 2x, light and dark, normal and Compact view.
- Home search: clicking the box, Ctrl+E, typing and Enter, Back, Close search, Quick access on and off.
- Ctrl+A in the rename box in Details, List, Tiles, Large icons, Content and search results, then Escape.
- Tooltips: hover, then Shift+F10, the Menu key, a right-click, a column title's right-click, F2 and a double-click into a folder.
- Search titles: three searches in a row, Back and Forward, and a split window.
- Home menus closed by Escape, by an item and by a click away, then the arrows and Enter.
- A fictional home with six pinned (cached) standard folders, a 3,000-file checkout, snap and hidden folders: "extra" from Home found the four files in OX-Test/Deb32/Logs and the one in Documents, and not the hidden one.
- Ctrl+A then Delete on Home after selecting a whole folder, with Quick access on and off, and Ctrl+A on This PC.
- Window size:
  - First start on 1366 × 768 at 1x, and at 1650 × 1050 at 200 %.
  - A saved size larger than the screen, and a saved size that fits.
  - Maximize and restore.
- No crashes or GTK criticals.

## Known limits

- A partly cached local folder is now walked rather than read from the cache, so on a very large home folder its results come in as the walk goes, as they already did for any folder with no cache at all.
- After a change of location, GTK may show the tooltip of the item that comes under a resting pointer once the pointer moves again. The old item's tooltip no longer stays.
- In the nested Weston used for testing, xdotool's right-click and the keyboard menus did not open menus on `main` either. On Wayland, hiding on Shift+F10 and the Menu key was checked; the menu cases were checked on X11.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
