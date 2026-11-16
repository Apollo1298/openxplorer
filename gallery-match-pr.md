# Title

feat(gallery): Show Gallery as Windows 11 does, with justified photo rows, a date scrubber and its own commands

# Body

Model: claude-opus-5-5

## Summary

Gallery now looks and works like Windows 11 File Explorer's Gallery (build 26200):

- **Navigation pane, tab and address bar.** "Gallery" sits right under Home with a blue picture glyph. The tab shows the same blue glyph, and the address bar shows the glyph and "Gallery". Typing "Gallery" in the address bar still opens it.
- **Photos.** Gallery shows the Pictures folder and its subfolders, newest first, at Medium icons. The status bar counts the photos ("6 items").
- **Justified rows.** The thumbnails keep each photo's shape, so a narrow screenshot is narrow and a panorama is wide. They sit in rows about 160 pixels tall at Medium icons that fill the width, with even 4-pixel gaps. Only the rows near the screen get widgets, so 10,000 photos scroll smoothly.
- **No month groups.** Instead there is one heading over the photos with the months on screen ("November 2026 – July 2025"). The heading follows the scrolling.
- **Date scrubber.** A scrubber at the right edge replaces the scroll bar. It marks where each year starts, has up and down arrows, and names the month under the pointer. A click or a drag on it jumps there.
- **Command bar.** It shows New (grayed), Cut, Copy, Paste, Rename, Share, Delete, Sort (grayed), View, Collection and "…". While photos are selected it also shows Set as background, Rotate left and Rotate right.
- **View menu.** It offers only Extra large, Large, Medium and Small icons, then the Details and Preview panes, then Show. Ctrl+mouse wheel steps through those four sizes only.
- **Collection menu.** It has Show all (the default, with a dot), one row per location, and Manage collection.
- **Gallery Locations dialog.** Manage collection opens it. It has the blue heading "Choose which folders to see in Gallery", Windows' explanation, and the "Gallery locations" list. Each folder shows its path, and Pictures shows "Default save location". Add… and Remove (grayed for Pictures) are under the list, then OK and Cancel. Nothing is saved until OK. Enter presses OK. A folder that does not exist is refused.
- **A photo's right-click menu.** The strip has Cut, Copy, Share and Delete. The menu has Open, Open with, Share with, Set as desktop background, Rotate right, Rotate left, Open file location, Add to Favorites, Compress to, Copy as path, Properties and Show more options. Windows-only app commands are left out.
- **Tooltip.** Hovering a photo shows "Path: name (folder)", "Item type: PNG File", "Date: …" and "Size: …".
- **Rotate left and Rotate right.** A JPEG is turned by its EXIF orientation alone, so its pixels stay byte-for-byte the same. The orientation is changed in place, or added without moving any existing EXIF data. A PNG's pixels are turned and saved as PNG again, which is lossless. The new file is written next to the photo and then renamed over it. No new dependency.
- **Sharp thumbnails.** Each photo on screen is drawn from a picture at least as large as its tile in device pixels, at 1x and 2x and at every View size, so text in a screenshot stays readable. The desktop's thumbnail cache only goes up to 256 pixels, and a Medium tile can be 800 pixels wide. So a tile first shows the cached thumbnail. Once it is on screen, the photo is decoded, shrunk once with gdk-pixbuf's best filter to exactly the tile's device pixels, and drawn pixel for pixel in its place. It is drawn as a plain texture node because GTK's software renderer draws a scaled texture through a surface of the widget's logical size, which at a display scale of 2 would halve the picture. A few decode at a time, within the app's picture limits. A decode is cancelled when its tile leaves the screen, and a View change never blanks or blurs a photo. A photo on a network keeps the cached thumbnail.
- **Honest loading progress.** Gallery cannot know how many photos it will find until every folder is read. While it lists, the address bar's progress sweeps instead of crawling toward an end it cannot know, and the status bar counts the photos found so far with "Loading…" ("4,800 items · Loading…").
- **Set as background** and **Set as desktop background** show grayed. The app has no opt-in way yet to set the desktop's background.

The month grouping from the first version of Gallery is removed. Other folders are unchanged.

## Behaviors and sources

| Behavior | Source |
|---|---|
| Gallery is a node in the navigation pane, right under Home, with a blue picture glyph | [Build 23435](https://blogs.windows.com/windows-insider/2023/04/14/announcing-windows-11-insider-preview-build-23435/); place and glyph as Windows 11 build 26200 shows them |
| The most recently taken photos appear at the top | [Windows configuration update, September 26, 2023](https://support.microsoft.com/en-us/topic/september-26-2023-windows-configuration-update-542780c2-594c-46cb-979d-11116fe164ba) |
| Pictures with its subfolders, "6 items" for six photos, no month groups | Windows 11 build 26200 |
| Justified rows that keep each photo's shape, about 160 pixels tall at Medium icons | Windows 11 build 26200 |
| One heading with the months on screen, following the scrolling | Windows 11 build 26200 |
| Date scrubber with years, arrows, a month on hover, click and drag | Windows 11 build 26200 |
| Command bar in Gallery, with Sort and New grayed, and the photo commands | Windows 11 build 26200 |
| View menu with only the four icon sizes, the panes and Show | Windows 11 build 26200 |
| Ctrl+mouse wheel resizes the thumbnails | [Build 23440](https://blogs.windows.com/windows-insider/2023/04/19/announcing-windows-11-insider-preview-build-23440/) |
| The Collection dropdown chooses which folders are shown | [Build 23435](https://blogs.windows.com/windows-insider/2023/04/14/announcing-windows-11-insider-preview-build-23435/); its items (Show all, each location, Manage collection) as Windows 11 build 26200 shows them |
| The Gallery Locations dialog | Windows 11 build 26200 |
| A photo's right-click menu | Windows 11 build 26200 |
| A tooltip on hover | [Build 23471](https://blogs.windows.com/windows-insider/2023/06/01/announcing-windows-11-insider-preview-build-23471/); its lines as Windows 11 build 26200 shows them |
| Sharp thumbnails at every size | Windows 11 build 26200 |
| Alt+P and Shift+Alt+P open the panes | [Build 22631.2191](https://blogs.windows.com/windows-insider/2023/08/10/announcing-windows-11-insider-preview-build-22621-2191-and-22631-2191/) |

## Not confirmed

These details are the app's own choices. They still need checking in Windows 11:

- The row heights at Small, Large and Extra large icons (100, 240 and 320 pixels here), and the 4-pixel gap.
- What the scrubber's arrows step by (one row here), the look of its position mark, whether it shows when every photo fits (hidden here), and whether it takes keyboard focus (no, like a scroll bar).
- Where Set as background, Rotate left and Rotate right sit in the command bar (after Collection here), and whether they show for several selected photos (yes here).
- Whether Paste works in Gallery (grayed here, as Gallery is no folder).
- What Compact view changes in Gallery (nothing here).
- The look of a hovered, selected or keyboard-focused photo, and whether photos show check boxes.
- What the heading shows when no photo on screen has a date (an empty heading row here).
- Which image types Windows rotates, and how it stores the turn (JPEG and PNG here, as described above).
- The date format of the tooltip, and its Path line for a photo in a subfolder (the photo's own folder here).
- The arrow keys, Page Up, Page Down, Home and End among the photos.
- The title of Add…'s folder picker ("Add a folder to Gallery"), whether network folders can be added (only local folders here), and whether Enter presses OK.

Not done: Set as background and Set as desktop background, which show grayed.

## How it was tested

- **Tests first for sharpness and progress.** Before the change, two window tests failed: every photo on screen must be asked for and drawn at no less than its tile's device pixels at Medium, Extra large, Small and Large icons, and Gallery's listing must show the sweeping progress and a "Loading…" count. A unit test checks the decode size for photos of every shape. They pass at 1x and with `GDK_SCALE=2`.
- **Tests first.** Fourteen window tests and a unit test for these behaviors were written before the change and failed on the earlier Gallery. They are in `window/tests/gallery_view.rs` and `window/tests/gallery_commands.rs`. Unit tests cover the justified layout, the heading text, the row heights and the rotation of JPEG and PNG files. All 55 Gallery-related tests pass.
- **Full suite.** Every native test executable and the doctests ran with `native/tools/check.py`'s isolation and were compared by name with unchanged main: 4,645 tests on this branch, 4,584 on main. The 13 failures on main also fail here: `admin_copy` (9), the Python location fixture, the FUSE export paths and the two sandboxed-thumbnailer tests, which need bubblewrap permissions this machine lacks. The `admin_delete` parent-path tests failed here only because the test user could not write inside the checkout. Given a folder it owns (`OX_TEST_TRUSTED_BASE`), all 71 pass. A stylesheet test caught a raw colour in the scrubber's month label, which was fixed, and the test passes now.
- **Other checks.** `native/parity/check.py`, the i18n check, the Markdown sync check, rustfmt, Clippy (`-D warnings`, and again with the pedantic flags) and `tools/audit-public-data.py` pass on the Gallery changes.
- **Real input.** The real app was driven with xdotool, using generated pictures only. It ran under Xvfb (X11) and under weston inside Xvfb (Wayland), at 1x and 2x, in light and dark, and in normal and Compact view: 8 configurations with 17 checks each, all passing. The checks cover:
  - clicking the navigation pane row and typing "Gallery"
  - the heading following wheel scrolling
  - hovering, clicking, dragging and the arrows of the scrubber
  - the command bar with and without a selected photo, and Sort opening nothing
  - Gallery Locations through the real folder picker: Add then Cancel, Add then OK (saved), and Remove then Enter
  - Rotate right and Rotate left on a PNG
  - no GTK criticals
- **Sharpness, measured.** A window test renders a tile of a small-print picture through the window's own renderer, with a 128-pixel thumbnail of it already cached. It compares the result with the original area-averaged to the tile, at Medium and Large icons, at 1x and at a scale of 2. With the earlier drawing it fails at 2x. In the real app, under X11 and Wayland at 1x and 2x, each tile is within 0.5 to 3.3 levels (out of 255) of the original shrunk to it, against 5.6 to 9.0 for the stretched 128-pixel thumbnail.
- **Text screenshots.** The captures include generated, fictional screenshots with small text. At Extra large icons the text is crisp at 1x and 2x.
- **10,000 photos in 100 folders.** The first photos show about 1 second after Enter. Page Down and the mouse wheel keep redrawing while the rest load. The status bar says "10,000 items", and a click on the scrubber jumps at once. Idle CPU is 0% after loading and after leaving.
- **Empty or missing Pictures.** Gallery shows "No photos to show" with no error.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
