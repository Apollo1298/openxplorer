# Title

feat(archives): Browse, open and extract 7z archives like ZIP and TAR

# Body

Model: claude-opus-5-5

## Summary

A `.7z` file now behaves like a ZIP or TAR file. Double-clicking it opens it like a folder, following the same "Double-clicking a ZIP" setting. You can browse its folders, sort, and see the Details columns. A file inside opens as a read-only private copy. Extract all (with "Show extracted files when complete") and Extract here unpack it, with the same progress, Cancel, Replace or Skip, and error handling as ZIP.

- The app reads the 7z headers itself and checks every count and size before the `sevenz-rust2` reader sees them. Listing a 7z decompresses nothing. The headers of the last few archives browsed are kept, so moving between folders of one archive reads only its first bytes again.
- File data is decoded by `sevenz-rust2` (same pinned version) on a thread of its own. A solid block is decoded once per extraction, not once per file. LZMA, LZMA2, Copy, Delta, the branch filters, BCJ2 (7-Zip's Ultra level for programs) and BZip2 are supported. BZip2 uses the crate's `bzip2` feature, which decodes with the `bzip2` crate the app already builds, so nothing is added to the build.
- Encrypted 7z archives are listed when their names are readable, and say plainly that encryption isn't supported. Windows 11 does the same. An archive with encrypted names says so before listing. Other methods (PPMd, Deflate) are listed and refused with a plain message.
- An archive that can't be read (damaged, encrypted names, any format) now shows an archive page in the tab: an archive icon, "This archive can't be opened" and the reason, not selected and without a Try again button that would fail the same way. Before, it showed the network "This location is unavailable" page.
- Unsafe paths, links and special files are hidden and stop an extraction, as for ZIP and TAR. A damaged or cut 7z is refused, and nothing is left behind.
- Fix for Compress to › 7z File: the pinned `sevenz-rust2` writer marked every folder as an "anti-item" (a delete marker), so 7-Zip and Windows dropped empty folders when they extracted the archive. Folders are now written as plain folders.

## Before, after and Windows 11

| | Before | After | Windows 11 |
|---|---|---|---|
| Double-click a .7z | Opens in the default app | Opens like a folder (or in the archive window), as the ZIP setting says | Opens like a folder ([Zip and unzip files](https://support.microsoft.com/en-us/windows/zip-and-unzip-files-8d28fa72-f2f9-712f-67df-f80cf89fd4e5): "Windows 11, version 24H2 supports ZIP, RAR. 7z and TAR archive formats") |
| Extract all / Extract here | Not offered for 7z | Same dialog and rules as ZIP | Extract All ([Zip and unzip files](https://support.microsoft.com/en-us/windows/zip-and-unzip-files-8d28fa72-f2f9-712f-67df-f80cf89fd4e5)) |
| Archive that can't be read | Network "This location is unavailable" page, reason drawn selected, Try again | Archive page: archive icon, "This archive can't be opened", the reason, no Try again | Not described on the Microsoft pages |
| Encrypted 7z | Default app | Listed when names are readable, then "isn't supported" | "it does not support operations on encrypted archive files" ([Zip and unzip files](https://support.microsoft.com/en-us/windows/zip-and-unzip-files-8d28fa72-f2f9-712f-67df-f80cf89fd4e5)) |
| Empty folders in a new 7z | Lost when 7-Zip or Windows extracts it | Kept | Creates 7z archives ([What's new in Windows 11, version 24H2](https://learn.microsoft.com/en-us/windows/whats-new/whats-new-windows-11-version-24h2): "Support for creating 7-zip and TAR archives") |

## Limits

- Encrypted 7z archives are refused plainly, as Windows 11 refuses encrypted archives ("it does not support operations on encrypted archive files", [Zip and unzip files](https://support.microsoft.com/en-us/windows/zip-and-unzip-files-8d28fa72-f2f9-712f-67df-f80cf89fd4e5)).
- 7z archives packed with Deflate or PPMd are listed and refused plainly. Decoding Deflate would switch flate2, which every ZIP and .tar.gz uses, to another backend; that is too large a change for these rare files.
- RAR archives still open in their default app.

## How it was tested

- New tests: 17 in `native/crates/ox-core/tests/archive_seven_zip.rs`, 1 in `native/crates/ox-core/src/archive/create.rs`, 5 in `native/crates/ox-app/src/window/tests/seven_zip_folder.rs`. They were written first and failed before their change: 14 core and all 5 app tests before the reader, the BZip2 test before the feature, the empty-folder test before the writer fix and the archive-page checks before that fix. All pass now. The BCJ2 test was added with BCJ2 support, and a forged-coder test guards that path.
- Archives made by 7-Zip 23 (solid, non-solid, LZMA, Ultra with BCJ2, PPMd, BZip2, AES, encrypted names, a symbolic link) were listed and extracted, and the results compared with 7-Zip's.
- Real keyboard and mouse input with xdotool on Xvfb, at 1x and 2x, light and dark: opening, browsing, sorting, opening a file inside, Extract all, a 10,000-file solid archive, Cancel, Replace and Skip, a full disk and a read-only destination. No crash, no GTK criticals and no data loss.
- `python3 native/tools/check.py`, `python3 native/parity/check.py`, rustfmt and Clippy.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
