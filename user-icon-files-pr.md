# Title

feat(icons): Read icons from Windows icon files the user chooses

# Body

Model: claude-opus-5-5

Relates to AKolenda/openxplorer#357. This adds one icon source, Windows icon files the user already has, and is meant to plug into the per-type icon overrides discussed there rather than compete with them.

This change reads icon files that the user supplies. It ships no Windows icons: nothing from Microsoft is in the repository, the package, the docs, the tests or the pictures.

## What it does

- **Settings › Appearance › Icons** gets **Icons from Windows icon files**, off by default, and **Windows icon files** with **Choose files…** (one or more `.dll`, `.mun` or `.exe`) and **Clear**. Only the paths are saved (`useIconFiles`, `iconFiles`); the row says how many icons the files gave.
- While it's on, the chosen files are read off the main thread. `imageres.dll.mun` and `shell32.dll.mun` (or the same names without `.mun`) draw the folder, Desktop, Downloads, Documents, Pictures, Music, Videos, Home, Gallery, This PC, a drive, Local Disk, the Recycle Bin (empty and full), ZIP folders, text files and generic files.
- Each icon uses the image that best fits its place and scale: 16/20 px in the navigation pane, 32/48 px in lists, 256 px in large views, PNG or bitmap entries.
- Anything missing, unreadable or broken keeps the built-in icon. Turning the setting off brings the built-in icons back at once.

## How

- `ox_core::windows_icons` is a small PE resource reader. It reads `RT_GROUP_ICON` and `RT_ICON` with bounds-checked `pread`s and caps the file size, sections, directory entries, images per group and image size. It decodes bitmaps itself and checks a PNG's header before anything decodes it. There's no unsafe code and no new crate: `flate2` and `crc32fast` were already dependencies, so `cargo-sources.json` is unchanged.
- Decoded images are cached as PNG files only in `~/.cache/winspace/windows-icons`, keyed by each file's path, size, time and inode. Folders of files no longer chosen are removed.
- The app asks one function, `icons::user_icons::texture_for`, before `ArtImage` draws its own icon; This PC's drive tiles use `user_icons::role_image`. A per-type override feature can sit in front of or behind this source.
- Local Disk gets its own `Art::SystemDrive`, which looks the same as before unless the files draw it.
- Parity: LOOK-029.

## Tests

Tests build tiny synthetic PE32/PE32+ files with synthetic test patterns inside the test. They cover parser bounds and refusals, every truncation, 3,000 seeded mutations, header bytes at their extremes, caps, name matching, the role table, the cache, fallback, the setting off by default and stored only when chosen, the best size per scale, and the Settings rows driving the navigation pane.

Run here with Rust 1.92.0: `python3 native/parity/check.py`, plus every stage of `python3 native/tools/check.py`. In this container `check.py` runs as root, so the app refuses to start and permission tests are bypassed. Those tests passed when rerun as an unprivileged user. Two Python-fixture tests fail here for reasons unrelated to this change: the container's Python 3.13 can't import the system `gi` module, and its `ipaddress` gives a different answer. GTK's file dialog itself isn't driven, and nothing was tried with real Windows files.
