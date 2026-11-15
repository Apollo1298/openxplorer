# Title

fix(properties): Make every Properties window work for shortcuts, the Recycle Bin, shared folders and the keyboard

# Body

Model: claude-opus-5-5

This pull request fixes what was broken or unlike Windows 11 in the Properties windows, found by opening every tab for files, folders, drives, shortcuts, several items, Recycle Bin items and items on a shared folder with real mouse and keyboard input.

In plain words:

- The Sharing tab works again on current Linux systems. It asked Samba with `net usershare info -l`, which Samba 4.15 and newer refuse, so every folder said "Folder sharing needs Samba..." and showed a raw "Invalid option -l: missing argument" line. It now asks with `--long`.
- An item in the Recycle Bin has real Properties: where it was deleted from (Origin), when (Deleted), its type, sizes and dates. Before, the window only said "Only local paths, smb:// locations and connected devices are supported in this build."
- Links and launchers (`.desktop` files) have a Shortcut tab after General, as a shortcut has in Windows: Target type, Target location, Target, and for a launcher Start in and Comment, plus Open File Location. Changes apply on OK or Apply only. A link is re-pointed by making a new link and renaming it over the old one, so it is never missing. A launcher keeps its other keys, comments, translations and permission to run. A link to a folder no longer gets the folder-only Sharing and Customize tabs, and shortcuts no longer show an "Opens with" row.
- Alt and an underlined letter press the buttons and tick the boxes, as in Windows dialogs: Alt+R Read-only, Alt+H Hidden, Alt+C Change..., Alt+A Apply, Alt+E Edit..., and so on. The letters show while Alt is held. Translations keep working, because no message text changed.
- Every Properties window is 380 by 520 pixels, title bar and border included. A page taller than that scrolls inside the window, never sideways.
- The name and Hidden are greyed for an item the user may not rename, instead of failing on Apply with "Permission denied".
- With several items selected, a link to a folder counts as a file, as one link's Properties show it. The totals are exact, not "≥".
- The Details tab keeps its photo, song and video rows after the app's file is replaced while it runs, as a package update does. Its reader process is now started through `/proc/self/exe`.
- Enter in a Shortcut tab field presses OK, as Enter in the name field does.

| Case | Before | After | Windows 11 |
|---|---|---|---|
| Sharing tab with Samba 4.15+ | "Invalid option -l: missing argument", Share... greyed | Shared/Not Shared, Share... and Advanced Sharing... work | Sharing tab works |
| Recycle Bin item | Error text only, four tabs | General tab with Origin and Deleted | General tab with Origin and Deleted (NOT CONFIRMED) |
| Link or `.desktop` launcher | General only; folder link had Sharing, Customize and "Opens with: No default application" | Shortcut tab with Target, Start in, Comment, Open File Location | Shortcut tab (NOT CONFIRMED) |
| Alt+letter | Nothing | Presses the control, underline shows with Alt | Alt + underlined letter (documented) |
| Window size | 402 x ~590 | 380 x 520 for every window | about 380 x 520 (owner rule) |
| Item in a folder the user cannot change | Hidden enabled, Apply fails "Permission denied" | Name and Hidden greyed | NOT CONFIRMED |
| Folder link + file selected | "≥ 167 bytes", "1 Files, 1 Folders" | exact bytes, "2 Files, 0 Folders" | shortcut counts as a file |
| Details after an update replaced the app | File rows only | All media rows | n/a |

Microsoft documents Alt + underlined letter for dialog boxes: https://support.microsoft.com/en-us/windows/keyboard-shortcuts-in-windows-dcc61a57-8ff0-cffe-9796-cb9706c75eec

## CHANGED OWNER DECISIONS

- Properties size: 402 by about 590 pixels (set in an earlier change for the tallest page) is now 380 by 520, the owner's rule for every Properties window. Tall pages, such as a drive's General under two rows of tabs, now scroll inside the window.
- A link to a folder no longer has the Sharing and Customize tabs, and no shortcut's General tab has "Opens with".
- An item in the Recycle Bin shows only a General tab.

## NOT CONFIRMED

These need a check in Windows 11, because Microsoft's documentation does not describe them:

- The exact access-key letters in each Properties tab (R, H, C, A on General and the footer; E and V on Security; S and D on Sharing; S, O, R on Previous Versions; O, S, F, R, I, T on Customize; D on a drive; F, T, S, O on the Shortcut tab).
- The Shortcut tab's rows and order (Target type, Target location, Target, Start in, Comment, Open File Location). Change Icon for a launcher is not offered yet.
- The rows of a Recycle Bin item's Properties, and that it has only General.
- Whether Windows' General tab of a shortcut shows "Opens with", and whether a shortcut to a folder has Sharing or Customize.
- Whether Tab stops on the read-only values (Location, Size and so on) of the General tab.
- Whether Windows refreshes an open Properties window when the file changes elsewhere. This app does not.

## How it was tested

- New tests, each failing on main first: `shares_are_listed_with_the_long_option_newer_samba_takes`, `an_item_that_cannot_be_renamed_greys_its_name_and_hidden`, `several_items_count_a_link_to_a_folder_as_a_file`, `an_item_in_the_recycle_bin_shows_where_it_came_from_and_when_it_was_deleted`, `alt_and_an_underlined_letter_press_the_properties_controls` (real keys through XTEST), `every_kind_of_properties_has_one_size` (now checks 380 x 520), the three tests in `window/tests/shortcut_tab.rs`, `the_reader_starts_the_running_program_even_once_its_file_is_replaced`, and unit tests in `properties/shortcut_tab.rs`.
- The whole `ox-app` test program ran as an unprivileged user on a private Xvfb display, once on this branch and once on unchanged main. Branch: 2470 passed, 8 failed. Main: 2366 passed, 8 failed. The 8 are the same tests on both: four Settings tests that need the retired Python app's sources, two thumbnail-sandbox tests and two sidebar-width tests.
- `cargo fmt --check`, `python3 native/parity/check.py`, the parity unit tests and `python3 native/tools/i18n.py check` pass. Clippy shows no findings in the changed code. The installed Clippy (1.97) is newer than the project's Rust 1.92 and reports new lints in files this change does not touch, so `python3 native/tools/check.py` stops at its Clippy step, on main as well. Its other steps were run through its own functions: the inventories, the icon guard, rustfmt, every test executable on its own private display and D-Bus bus, and the doctests (which pass). Run as root, because only root could use the Rust toolchain here, some test executables fail because root may write anywhere and the app refuses to start as root. The four such ox-core executables were also run on main and fail the same tests there.
- By hand, with real mouse and keyboard input through xdotool on Xvfb at 1x and 2x, in the light and dark themes: a text file, a photo, a song, a video, a PDF, a script, a small folder, a folder of 50,000 files, a folder with an unreadable subfolder, a loop-mounted ext4 drive, a read-only squashfs drive, a tmpfs mount, the root drive, a link to a file, a link to a folder, a `.desktop` launcher, several items, a Recycle Bin item and a file on a local Samba share. Sharing was turned on and off through Advanced Sharing against a real Samba 4.19.
- Under weston, natively on Wayland at scale 1 and 2, the Properties window draws at 380 x 520 logical pixels. Xwayland in a headless weston crashes on synthetic pointer input, so the keyboard and mouse checks under weston could not be made.
- Pictures of every tab for a file, a shortcut, a launcher, a folder, a drive and several items, at 1x and 2x in both themes, were taken with the repository's capture library and the fictional demo tree.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
