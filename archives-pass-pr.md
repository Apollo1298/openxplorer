# Title

fix(archives): Ask for ZIP passwords, read backslash names and write ZIP64

# Body

Model: claude-opus-5-5

## Summary

Four archive differences from Windows 11 File Explorer are fixed:

- **Password-protected ZIPs.** A ZIP encrypted with the traditional ZIP method is still browsed without a password. Opening a file in it, Extract all, Extract here and copying or dragging items out now ask "Password needed". A wrong password asks again with "The password is incorrect. Try again.", Cancel stops without a message, and nothing is written until the password is right. The password that worked is kept in memory for that archive until the window closes, so a second file does not ask again. It is never saved, logged or shown. AES-encrypted ZIPs are listed, and the Extract dialog now says plainly that they need an archive manager. Creating password-protected ZIPs is not added.
- **Backslashes in ZIP names.** Some Windows tools write `Docs\a.txt` instead of `Docs/a.txt`. Those files were hidden and Extract all refused the whole archive. Backslashes are now read as folder separators. Every rule for unsafe paths still applies to the folders they give, so the app still rejects unsafe paths.
- **Large ZIPs.** Compress to ZIP stopped with "ZIP files over 4 GiB or with more than 65,535 items need an archive manager." The writer now adds ZIP64 records where a value needs them.
- **Extract dialog wording.** The check box reads "Show extracted files when complete", as in Windows.

A Linux file whose name holds a backslash, compressed to ZIP, now extracts as a folder and a file, as it would on Windows.

No new dependency.

## Before and after

| Case | Before | After | Windows 11 |
|---|---|---|---|
| Open a file in a password ZIP | "Encrypted ZIP members require an archive manager." | "Password needed" prompt, retry on a wrong password, Cancel says nothing | Asks for the password (see NOT CONFIRMED) |
| Extract all / Extract here on a password ZIP | Refused: "This ZIP has a password, so OpenXplorer cannot extract it." | Same prompt; nothing written until the password is right | Asks for the password (see NOT CONFIRMED) |
| Copy or drag items out of a password ZIP | Refused | Same prompt | Asks for the password (see NOT CONFIRMED) |
| AES-encrypted ZIP | "This ZIP has a password…" | "This ZIP uses an encryption method OpenXplorer cannot open. Use an archive manager." | NOT CONFIRMED |
| ZIP with `Docs\a.txt` names | Files hidden; Extract all refused | `Docs` folder shown and extracted | Backslash is a path separator and cannot be part of a file name ([Naming Files, Paths, and Namespaces](https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file)); how Explorer shows such ZIPs is NOT CONFIRMED |
| Compress more than 65,535 items, or over 4 GiB, to ZIP | Stopped with an error | ZIP64 written; Info-ZIP, Python and the app read it | NOT CONFIRMED |
| Extract dialog check box | "Show extracted files when finished" | "Show extracted files when complete" | "Show extracted files when complete" |

## CHANGED OWNER DECISIONS

- ARC-006 and ARC-016: encrypted ZIP members were always refused. Traditional ZIP encryption now asks for the password; AES and strong encryption stay refused, with a clearer message.
- ARC-004 and ARC-014: a backslash in a member name made it unsafe (hidden, and Extract all refused). It now separates folders; the unsafe forms it gives (`..\x`, `\\server\share`, `C:\x`) are still rejected.
- ARC-023: the 65,535-item and 4 GiB limits of Compress to ZIP are gone. One compression now plans at most 1,000,000 items (ZIP and TAR alike) to keep memory bounded.
- ARC-009: the check box text follows Windows' words.

## NOT CONFIRMED

- Microsoft's [Zip and unzip files](https://support.microsoft.com/en-us/windows/zip-and-unzip-files-8d28fa72-f2f9-712f-67df-f80cf89fd4e5) page says Windows 11 "does not support operations on encrypted archive files". Whether Windows 11 24H2 still asks for the password of a traditional (ZipCrypto) ZIP, and the exact words of its prompt, need checking on Windows.
- Whether Windows remembers a ZIP password between files of the same archive.
- How File Explorer shows ZIP names written with backslashes.
- Whether Windows writes ZIP64 for archives over 4 GiB or 65,535 items.
- The Details columns Windows shows inside a ZIP (compressed size, ratio, password protected).
- Whether Windows opens any file type from inside a ZIP; the app still opens only documents, images and media.
- 7z and RAR browsing and extraction are not added here: the same page says Windows 11 24H2 supports ZIP, RAR, 7z and TAR, but the app links no 7z or RAR reader yet.

## How it was tested

- New core tests: `archive_password.rs` (missing and wrong passwords, Info-ZIP `zip -P` archive with data descriptors, AES refused, copy out), `archive_backslashes.rs`, `archive_zip64_write.rs` (65,536 files read back by Python's zipfile and the app), unit tests for the cipher, the password type, name decoding and the ZIP64 writer (checked with Python's zipfile). Each new test failed before its fix.
- New window tests: `window/tests/archive_passwords.rs` (open, Extract all, Extract here, copy, Cancel) and `a_zip_written_with_backslashes_shows_its_folders`, on Xvfb and the password tests also natively on headless weston.
- By hand on a private Xvfb display with fictional files, mouse and keyboard input, at 1x light and 2x dark: the password prompt, wrong then right password, the copy opening in its application, Extract here, the AES refusal, a backslash ZIP, a 10,000-entry ZIP, a truncated ZIP and Compress to ZIP with an empty folder, a link to a file outside and a name conflict.
- A real 4.3 GiB file compressed to ZIP (release build): `unzip -t` reports no errors, Python reads the right size, and the app's extraction is byte-identical.
- Disk full, a read-only destination and a 200 MiB zip bomb: clear error, nothing left behind.
- `python3 native/tools/check.py`, `python3 native/parity/check.py`, rustfmt and Clippy with the workspace lints: parity, i18n, docs sync, the public-data audit, rustfmt and Clippy pass. `check.py` stops at its first failing test binary, and here `location_fixture_drift` fails on unchanged main as well (this machine's Python parses one SMB address differently), so every test binary was also run in the same isolation with `--no-fail-fast`, on this branch and on unchanged main, and the failures compared by test name. No archive test fails. The failures that remain also fail on main in this environment (Python-app comparison tests that need `check.py`'s extracted sources, administrator-copy and sandboxed-thumbnailer tests, a few settings and timing tests).
- The website's pictures were not changed; no website build was run.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
