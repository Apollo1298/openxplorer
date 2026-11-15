# Title

fix(keys): Keep the keyboard where Windows keeps it in dialogs, menus, panes and the folder tree

# Body

Model: claude-opus-5-5

This goes through Microsoft's list of File Explorer and dialog box keys and fixes the places where the keyboard got lost, went somewhere Windows would not send it, or a key did nothing. The biggest one: in every question dialog (Delete File, Rename File, Confirm Save As and the others), Tab past the last button gave the keyboard to nothing, and after that no Tab or Shift+Tab brought it back, so a keyboard user had to close the dialog with Escape. Tab now goes round the buttons in both directions.

Also in this change: F4 opens a drop-down list in dialogs and Properties windows; F6 reaches both panes of a split tab; dialogs shown inside the window keep Tab inside themselves and give the keyboard back to where it was when they close; closing a crumb's folder menu gives the keyboard back to the crumb; Left and Right with nowhere to go stay in the navigation pane and the folder tree instead of jumping to the next folder or to the pane's resize handle; and the Keyboard shortcuts window lists the keys from Microsoft's list that already worked but were missing there, with Enter and Backspace named as Windows names them.

All Microsoft quotes are from https://support.microsoft.com/en-us/accessibility/windows/keyboard-shortcuts-in-windows (Windows 11 tab).

| Key or focus behavior | Before | After | Microsoft |
|---|---|---|---|
| Tab and Shift+Tab in a question dialog (Delete File, Rename File, Confirm Save As, message boxes) | After the last button the keyboard went to nothing (GTK 4.14 with the dialog's hidden title bar); a selectable message or a body without controls then claimed every Tab without taking the keyboard, so it never came back | Tab and Shift+Tab go round the dialog's controls; a body is a Tab stop only when it has more to show than fits, and then draws the focus ring | "Tab: Move forward through options", "Shift+Tab: Move back through options" (Dialog box keyboard shortcuts) |
| F4 on a drop-down list (Folder Options, Map network drive, Create archive, the Open and Save dialog's file types) | Nothing; GTK opens a drop-down with Space or Enter only | Shows the list's items; elsewhere F4 keeps its meaning, such as the browser window's address bar | "F4: Display the items in the active list." (Dialog box keyboard shortcuts) |
| F6 in a split tab | Reached only the pane in front, so the other pane could not be reached with F6 | Visits each pane, the left one first, as Tab does | "F6: Cycle through elements in the window." (File Explorer keyboard shortcuts) |
| Tab in a dialog shown inside the window (Create archive, Extract, Confirm Save As) | Went on to the tab strip and the title bar's Minimize, Maximize and Close | Stays inside the dialog, as in a dialog window | "Tab: Move forward through options" (Dialog box keyboard shortcuts) |
| Closing a dialog shown inside the window | The keyboard always went to the file list, even from Confirm Save As's No | The keyboard goes back to the control that had it, such as File name; to the file list when that control is gone | Not stated by Microsoft; the existing rule that a dialog gives focus back to where it was (ACC-005) |
| Escape on a crumb's folder menu (Down on a crumb) | The keyboard went on to the search box | The keyboard goes back to the crumb | Not stated by Microsoft; same rule as above |
| Left or Right with nowhere to go in the navigation pane (Right on Local Disk, Left or Right on Home) | GTK moved the keyboard to the pane's resize handle, where the next arrow resized the pane | The keyboard stays on its row | "Left arrow: Collapse the current selection (if it's expanded), or select the folder that the folder was in." "Right arrow: Display the current selection (if it's collapsed), or select the first subfolder." (File Explorer keyboard shortcuts) |
| Right on an open folder without subfolders in the folder tree | Moved to the next folder down, as if it were a subfolder | Stays on the folder | "Right arrow: ... or select the first subfolder." (File Explorer keyboard shortcuts) |
| Keyboard shortcuts window | Did not list Backspace, Shift or Ctrl with the arrows, Ctrl+Space, Home, End, Left and Right in the navigation pane, Alt alone or Ctrl with the scroll wheel, though they work; showed Alt+Return | Lists them in Microsoft's words; shows Alt+Enter and Backspace | "Alt+Left arrow / Backspace: Navigate to the previous folder", "Shift+Arrow keys: Select multiple items", "Ctrl+Arrow key (to move to an item)+Spacebar: Select multiple individual items", "Ctrl+mouse scroll wheel: Change the size and appearance of file and folder icons", "Home" and "End" (File Explorer keyboard shortcuts) |

The user guide's Interface page says the same in one paragraph, and the translation template has the new strings. The snapshot hook gains a `tab-key=<count>` scene step, used to picture where Tab puts the keyboard in a dialog.

## CHANGED OWNER DECISIONS

- Navigation pane (SIDE-033): Left or Right with nowhere to go used to be "left to the list", which in GTK moved the keyboard out of the pane to its resize handle. It now keeps the keyboard on its row. The two assertions of `left_and_right_on_a_section_head_collapse_and_expand_it` that expected the key to be left to the list were changed accordingly.

## NOT CONFIRMED

These are not described in Microsoft's documentation, so they were left as they are and are listed in native/BACKLOG.md:

- Page Up and Page Down in List view follow GTK's column layout, so with every item in one column they do nothing.
- The Properties windows' values (Type, Location, Size and so on) and the title bar's Close are Tab stops.
- Each Settings category is a Tab stop, and the title bar's buttons are reachable with Tab on the Settings page.
- Each Home tile is a Tab stop of its own; the arrows also move between them.
- Delete File and Rename File have no Alt+letter keys and draw no underlined letters.
- A folder opened from the command line has Home before it in Back's history.
- Choose Details' title bar Close is a Tab stop.
- The type-ahead prefix resets after 1 second of no typing.
- Pointer and keyboard share one highlight in a menu, so Up from the first row starts from the row under a resting pointer.

## How it was tested

- Each fix has a test committed first that failed before the fix: `tab_goes_round_a_questions_buttons_and_never_into_its_text`, `f4_shows_the_items_of_a_drop_down_in_folder_options`, `f4_shows_the_items_of_a_drop_down_in_a_dialog`, `f6_visits_both_panes_of_a_split_tab`, `closing_a_crumbs_menu_gives_the_keyboard_back_to_the_crumb`, `tab_stays_in_a_dialog_shown_inside_the_window`, `no_in_confirm_save_as_gives_the_keyboard_back_to_file_name`, `left_and_right_with_nowhere_to_go_keep_the_keyboard_on_the_row`, `the_shortcuts_window_lists_the_keys_that_work_without_a_binding` and `right_on_a_folder_without_subfolders_stays_on_it_in_the_folder_tree`. `tab_goes_round_the_sign_in_dialog` was added as a regression test and passed before and after.
- Real key presses with xdotool on Xvfb with a window manager, in a private D-Bus session with a fictional home folder, reading where the keyboard went through the accessibility tree: 43 keyboard checks (list keys, selection keys, F2 to F6, F11, Alt and Ctrl keys, tabs, file commands, menus, dialogs, the Keyboard shortcuts window), each passing at 1x and 2x, light and dark, normal and Compact view, with no GTK criticals. Further real-key walks through Properties, Folder Options, the Open and Save dialogs, Replace or Skip, Create archive, Extract, Open with, the Home page, This PC, the Recycle Bin, split view, the address bar, the search box and open menus.
- Under Weston the app ran as a Wayland client at 1x and 2x without criticals; key presses could not be sent there (Xwayland would not start in the headless test environment).
- Pictures of the Keyboard shortcuts window and of the focus ring after Tab in Delete File, Map network drive and the Keyboard shortcuts window, at 1x and 2x, light and dark, through tools/native_capture.py with its fictional demo tree.
- `python3 native/parity/check.py`, the message template check, rustfmt and Clippy with `-D warnings` pass. The stages and test executables of `python3 native/tools/check.py` were run in its isolation (Python 3.12): all pass except 9 `admin_copy` tests and 2 sandboxed thumbnailer tests, which fail the same way on unchanged `main` in the test container, and `no_tooltip_stays_over_a_menu_the_rename_box_or_the_next_folder`, which waits for a tooltip that never shows there and failed the same way before this change. Two tests that failed only in the full run pass on their own. `the_keyboards_menu_opens_at_the_focused_item_of_a_selection` takes about 150 seconds there, on `main` too, so it was run on its own and passes.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
