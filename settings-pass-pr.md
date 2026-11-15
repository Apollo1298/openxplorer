# Title

fix(settings): Name folder options as Windows does, find them in Settings and make Folder Options work from the keyboard

# Body

Model: claude-opus-5-5

Settings and Folder Options set several of the same preferences, but Settings named them differently from Windows, left out hidden files and file name extensions, and its search did not find Folder Options' choices by the words Windows users type. Typing a two-word search on the Settings page lost its first word, and Folder Options could not be finished from the keyboard. This change:

- titles the shared rows with Folder Options' wording, so each option has one name in the app;
- adds "Show hidden files, folders, and drives", "Hide extensions for known file types" and an Open Folder Options button to Settings > Files & folders > Folder views, in step with Ctrl+H, View > Show and Folder Options;
- makes the settings search find Folder Options by its choices' Windows names (status bar, single-click, preview pane, clear File Explorer history, always search file names and contents, and so on);
- moves the keyboard into the settings search when typing on the page starts one, so the space in "hidden files" no longer chooses a category and ends the search;
- keeps Folder Options in step while it is open: a choice not changed there shows what Settings, the View menu, Ctrl+H or another window saved meanwhile, and a choice changed there keeps the change for OK;
- gives Folder Options' choices and push buttons access keys (Alt and the underlined letter), distinct on each tab;
- makes Enter press OK, the default button, wherever the keyboard is in Folder Options except on a push button, as in Windows. Before, the tab, check boxes and the start-place list took Enter themselves, so after Alt and a letter the dialog could not be closed with its choices saved.

No persisted key, application ID or schema changes. Every option keeps its default.

## Options fixed

| Option | Before | After | Windows wording |
|---|---|---|---|
| Full path in the title bar (Settings > General > Address bar) | "Show the full path in the title bar" | "Display the full path in the title bar", the Folder Options label | "Display the full path in the title bar" (Windows 11 Folder Options > View; not found on a Microsoft page, see NOT CONFIRMED) |
| Each folder's view (Settings > Files & folders) | "Remember each folder's view" | "Remember each folder's view settings" | Folder Options > View label (not found on a Microsoft page, see NOT CONFIRMED) |
| Start place (Settings > General > Start up) | "New windows open at" | "Open OpenXplorer to" | "Open File Explorer to:", named for the app as Folder Options already does: https://support.microsoft.com/en-us/windows/file-explorer-in-windows-ef370130-1cca-9dc5-e0df-2f7416fe1cb1 |
| Hidden files (Settings) | not in Settings; a search for "hidden files" found nothing useful | "Show hidden files, folders, and drives" switch | "Show hidden files, folders, and drives": https://support.microsoft.com/en-us/windows/file-explorer-in-windows-ef370130-1cca-9dc5-e0df-2f7416fe1cb1 |
| File name extensions (Settings) | not in Settings | "Hide extensions for known file types" switch, on by default as before | "Hide extensions for known file types": https://support.microsoft.com/en-us/windows/file-explorer-in-windows-ef370130-1cca-9dc5-e0df-2f7416fe1cb1 |
| Folder Options (Settings) | no way to reach it from Settings; its choices were not found by the settings search | "Folder Options" row with an Open Folder Options button, found by its choices' names | "Change folder and search options" opens Folder Options in Windows: https://support.microsoft.com/en-us/windows/file-explorer-in-windows-ef370130-1cca-9dc5-e0df-2f7416fe1cb1 |
| Restore previous folder windows (Settings > Start up) | not found by Windows' name | found by "restore previous folder windows" | Folder Options > View label (NOT CONFIRMED) |
| Settings search typed from the category list | "hidden files" searched "files" | the whole phrase is searched | app-only |
| Folder Options while open | showed values from when it opened, even after Ctrl+H or Settings changed them | unchanged choices follow; changed ones keep the change | app-only (Windows' dialog does not have Settings beside it) |
| Folder Options access keys | none | Alt+A, Alt+W (Browse folders), Alt+T (Open OpenXplorer to), Alt+L (Clear), Alt+O (Apply to Folders), Alt+R (Reset Folders), Alt+D (Restore Defaults) | letters NOT CONFIRMED |
| Enter in Folder Options | did nothing on a tab, check box or the start list | presses OK | Enter presses a dialog's default button in Windows (NOT CONFIRMED for this dialog) |

## CHANGED OWNER DECISIONS

None. Every default stays as it was.

## NOT CONFIRMED

These are Windows 11 Folder Options labels and behaviors that no Microsoft page I could read states; they need checking on Windows 11:

- "Display the full path in the title bar", "Remember each folder's view settings", "Restore previous folder windows at logon", "Show status bar", "Show pop-up description for folder and desktop items", "Show preview handlers in preview pane", "Always show icons, never thumbnails", "When typing into list view", "Always search file names and contents (this might take several minutes)", "Browse folders", "Apply to Folders", "Reset Folders", "Restore Defaults", "Show recently used files", "Show frequently used folders", "Clear File Explorer history".
- The access-key letters Windows uses in Folder Options, and whether Enter on a focused tab or check box presses OK there.

## How it was tested

- A failing test per fix, run on unchanged main first (all failed there), then passing here: `windows_folder_option_words_find_their_settings`, `folder_views_rows_follow_hidden_files_and_extensions_and_open_folder_options`, `every_setting_of_the_earlier_layout_is_on_one_of_the_new_pages`, `typing_on_the_category_list_moves_the_keyboard_into_the_search`, `folder_options_follows_preferences_changed_while_it_is_open`, `folder_options_choices_and_buttons_have_access_keys`, `folder_options_has_the_general_view_and_search_tabs_of_windows`, `real_keys_choose_options_and_enter_presses_ok` (real keyboard through XTEST).
- `python3 native/tools/check.py` and `python3 native/parity/check.py`, `python3 native/tools/i18n.py check`, `python3 tools/sync-docs.py` (results below).
- Real input with xdotool on a private Xvfb display at 1x and 2x, fictional files only: Ctrl+, opens Settings; typing "hidden files", Enter and Space turn hidden files on; "file extensions" turns extensions on; Enter on Open Folder Options opens it; Alt+W, Alt+D (Restore Defaults), Alt+W and Enter save "Open each folder in its own window" and close it; Escape closes it without saving; after a restart every choice is still saved. No GTK criticals.
- Pictures of every Settings page and Folder Options tab at 1x and 2x, light and dark, and in Simplified Chinese, through the repository's capture module.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
