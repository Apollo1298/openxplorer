# Title

fix(dialogs): Keep the keyboard where Windows' Open and Save dialogs keep it

# Body

Model: claude-opus-5-5

Other applications' Open and Save dialogs had several keyboard problems that a Windows user would notice at once:

- After answering **No** (or Escape) to Confirm Save As, the keyboard went to the file list instead of File name. The next Enter then opened the first folder there, or picked the file under the keyboard, instead of asking again.
- **Enter did nothing** in a Select folder dialog showing an empty folder. The Open, Save or Select folder button is now the dialog's default button, so Enter presses it when nothing else takes the key.
- **Tab to Save or Open showed no ring.** The ring is drawn in the accent color, which is also the button's fill. It is now drawn just outside that button.
- **Ctrl+Shift+N did nothing from File name**, where a Save dialog starts. The folder name typed next replaced the file name, so Enter saved a file named after the folder. It now makes a new folder from there too.
- **Alt+N and Alt+T did nothing.** "File name:" and "Save as type:" now have them, as Windows underlines those letters.
- **Y and N alone** now answer Confirm Save As, as the app's other Windows-style message boxes already do.
- **Several check boxes from the calling app** (an office suite's "Automatic file name extension", "Save with password", "Edit filter settings") each took a full row and left the file list a sliver. They now share one row and wrap only when the dialog is narrow; a list keeps its own row.

| Case | Before | After | Windows |
|---|---|---|---|
| No in Confirm Save As, then type a name and press Enter | Keyboard in the file list; Enter opened a folder | File name has the keyboard with the name selected | A dialog gives focus back to the control that had it ([WM_ACTIVATE](https://learn.microsoft.com/en-us/windows/win32/dlgbox/dlgbox-programming-considerations)) |
| Enter in a Select folder dialog showing an empty folder | Nothing | Chooses that folder | Enter presses the default push button ([same page](https://learn.microsoft.com/en-us/windows/win32/dlgbox/dlgbox-programming-considerations)) |
| Tab to Save or Open | No visible ring | Ring just outside the button, light and dark | NOT CONFIRMED (ring style is C29) |
| Ctrl+Shift+N with the keyboard in File name | Nothing; the folder name then replaced the file name | New folder, named in place | NOT CONFIRMED from File name |
| Alt+N / Alt+T | Nothing | File name / type list | NOT CONFIRMED (letters) |
| Y or N alone in Confirm Save As | Nothing (Alt+Y/Alt+N only) | Answers Yes or No | NOT CONFIRMED for this box |
| Three check boxes and a list from the caller | Four rows under File name | Check boxes on one row, list on its own | NOT CONFIRMED ([IFileDialogCustomize](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialogcustomize) says only that layout follows the order controls are added) |

## CHANGED OWNER DECISIONS

None. The ring keeps the accent color of C29; only its place on the accent-filled accept button changes.

## NOT CONFIRMED

Check these in Windows 11's Save As dialog:

- Ctrl+Shift+N with the keyboard in File name makes a new folder.
- The underlined letters are N in "File name:" and T in "Save as type:".
- Confirm Save As answers to Y and N without Alt.
- After No in Confirm Save As, File name has the keyboard with the whole name or only the part before the extension selected.
- How Windows lays out several check boxes an application adds.
- Where Windows draws the focus ring on the accent-filled Save button.

Not changed: the portal's `modal` option is still ignored. A test compositor with xdg-foreign was not available to check a non-modal dialog.

## How it was tested

- Each fix has a test that fails on unchanged main and passes here (`native/crates/ox-app/src/window/picker.rs`): `after_confirm_save_as_the_keyboard_is_back_in_file_name`, `enter_that_nothing_takes_presses_the_accept_button`, `the_keyboard_ring_on_the_accept_button_shows`, `ctrl_shift_n_makes_a_new_folder_from_file_name`, `the_callers_check_boxes_share_one_row`, `alt_n_and_alt_t_reach_file_name_and_the_type_list`.
- End to end, on a private X display (Xvfb): the real xdg-desktop-portal on a private D-Bus bus routed FileChooser to the app, which it started by D-Bus activation. The calls came from a script that calls the portal as Chromium and Electron do, and from real GTK 3 (FileChooserNative with choices) and GTK 4 (FileDialog) apps. The keys and clicks were real (xdotool). Every case in the table except Y and N alone (covered by the unit test) fails with the unchanged main build and passes with this one. Also checked, with fictional files: open and save with filters and current_filter, quoted lists, ~ paths, directory mode with accept_label, current_file, overwrite, a read-only folder, Escape in every part of the dialog, non-UTF-8 and very long names, two dialogs at once, the caller exiting, the remembered size, and a Samba share (smbd on localhost; without FUSE in the test machine the dialog refuses it and says why).
- `python3 native/tools/check.py`: the parity tests and inventory check, the i18n check, the tools' tests, the drawn-icon guard, rustfmt and Clippy pass. Every test executable was then run in the same isolation. The failures match unchanged main by test name and come from the test machine: 9 `admin_copy` tests, 2 sandboxed-thumbnailer tests (no user namespaces), and a Python clipboard check outside check.py.
- `python3 native/parity/check.py` passes.
- Under Wayland (headless weston), the dialog tests fail only where headless weston never gives a window the keyboard: the same 4 as main, plus the new ring test.
- Pictures in light and dark, at 1x and 2x, normal and Compact view, were taken with the app's snapshot hook through `tools/native_capture.py` and with the tests' capture hook.
- Not run: a share through GVfs FUSE, dragging a file into the dialog, and attaching the dialog to its caller on Wayland (weston has no xdg-foreign).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
