# Title

fix(ui): Search from Home and fix rename, tooltip, search title, Home focus and window size bugs

# Body

Model: claude-opus-5-5

This fixes six bugs that a Windows user runs into in the first minutes.

- **Search from Home.** With "Open Quick access on Home" on, the search box on the Home page was grayed out, so clicking it, Ctrl+E, F3 and typing did nothing. Windows says "To find relevant files from your PC and the cloud, search from File Explorer Home." The box now reads "Search Home" and searches the home folder and its subfolders, with the usual results view. The tab reads "<words> - Search Results in Home", and the status bar counts the results. Back and Close search return to Home.
- **Ctrl+A while renaming.** Ctrl+A in the rename box selected every item in the folder, and Escape left them selected, so a following Delete would recycle the whole folder. The list or grid around the box took the key for its own Select all. The rename box now keeps Ctrl+A and selects its own text only. Outside the box, Ctrl+A still selects every item.
- **Tooltips over menus, the rename box and the next folder.** GTK 4 keeps a tooltip on screen until the pointer moves, so a file's tooltip covered the menu that Shift+F10, the Menu key or a right-click opened. A column title's tooltip covered its menu, the hovered row's tooltip covered the rename box, and a folder's "Date created" tooltip stayed over the folder that a double-click opened. Now any key or button pressed in the window hides the tooltip, and so does every change of location. No new tooltip shows until the pointer moves. Tooltips also stay off while a menu, another popover or the rename box has the keyboard.
- **Search title.** A tab kept the first search's words ("a?b - Search Results in Documents") after the user typed on to a new search. The tab and the window title now follow every new search.
- **Keyboard focus after a Home menu.** After a Home tile's or row's menu closed, the arrows and Enter did nothing. The keyboard now goes back to the tile or row, whether the menu closed with Escape, by choosing an item that stays on Home, or by a click away.
- **Window larger than the screen.** On Wayland, GTK fits a new window to the whole monitor but not around the panels. At 200 % on a 1650 × 1050 screen (825 × 525 logical pixels), the window ran off the right and bottom edges. A new window's size, saved or default, is now fitted to the monitor in logical pixels, with room left for panels on Wayland. A saved size that fits is kept, and the user's saved size is not overwritten by the fitted one. On X11, GTK already fits windows to the work area.

| Fix | Before | After |
|---|---|---|
| Search from Home | Search box grayed out on Home; click, Ctrl+E, F3 and typing did nothing | "Search Home" searches the home folder and subfolders; "zebra - Search Results in Home", "1 result"; Back and Close search return to Home |
| Ctrl+A while renaming | Every item in the folder selected ("15 items selected"), still selected after Escape | Only the name's text is selected; "1 item selected" before and after Escape; Ctrl+A outside the box selects all |
| Tooltips | A row's or column title's tooltip covered its menu; the hovered row's tooltip covered the rename box; a folder's tooltip stayed over the next folder | Hidden on any key or click and on every change of location; none shows until the pointer moves; off while a menu or the rename box has the keyboard |
| Search title | Tab and window title kept the first search's words | Each new search's words, also after Back, Forward and in a split pane |
| Home menu focus | After Right, Shift+F10, Escape on a tile, Left and Enter did nothing | Left and Enter work after Escape, after choosing an item and after a click away |
| Window size | 1320 × 810 window on an 825 × 525 logical screen ran off the right and bottom edges | Fits the screen below the panel; a saved size that fits is kept; maximize and restore work |

## How it was tested

**Tests first.** Each fix has a test that failed on unchanged `main` and passes now:

- `searching_from_home_searches_the_home_folder_and_its_subfolders`: the box was disabled.
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
  - All 2,351 ox-app tests pass except the two sandboxed-thumbnailer tests, which also fail on `main`.
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
- Window size:
  - First start on 1366 × 768 at 1x, and at 1650 × 1050 at 200 %.
  - A saved size larger than the screen, and a saved size that fits.
  - Maximize and restore.
- No crashes or GTK criticals.

## Known limits

- When a pinned folder below the home folder is in the search cache, a search from Home (as from the home folder before) lists the home folder's own items plus the cached folders' matches, not a full walk of every subfolder. This is the existing "Current folder + cached subfolders" rule, unchanged here.
- After a change of location, GTK may show the tooltip of the item that comes under a resting pointer once the pointer moves again. The old item's tooltip no longer stays.
- In the nested Weston used for testing, xdotool's right-click and the keyboard menus did not open menus on `main` either. On Wayland, hiding on Shift+F10 and the Menu key was checked; the menu cases were checked on X11.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
