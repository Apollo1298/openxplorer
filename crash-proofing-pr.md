# Title

fix(native): Decode pictures in a sandboxed child so no file can crash the app

# Body

Model: claude-opus-5-5

## Summary

Windows Explorer never dies because of a file. OpenXplorer could: every picture shown in the Preview and Details panes, every thumbnail and every custom icon was decoded by gdk-pixbuf's C loaders inside the app, and one malformed file could abort, crash or hang the whole window. Opening a crafted `.7z` as a folder also crashed it.

- **Pictures decode in a child process.** `picture_guard` starts the app again with `--ox-decode-picture`, streams the picture's bytes to it and reads the pixels back. The child runs the same guarded decoding as before. Where bubblewrap works it runs in the thumbnailers' sandbox, with no network, no session bus and none of the user's files. It also limits its own CPU, memory, file size, core dumps and open files. The app kills it when the selection moves on, or after the picture's time limit plus 3 seconds. A child that crashes, hangs or writes nonsense just means "No preview available" or the plain icon. The app checks every size in the answer before using it.
- **Every caller goes through it:** the Preview pane, the Details pane picture, thumbnails, cached thumbnails, thumbnailer and PDF page output, custom folder icons, and the gallery's picture sizes. Properties' **Change Icon** pickers used to decode on the main thread, and PNG rotation used to decode in the app; both now go through it. The gallery reads picture sizes with a pure-Rust header reader where the format allows.
- **7z:** a forged files record could index past its bits and panic. The archive worker re-raised that panic on the main thread, which aborted the app. The read is fixed. The worker now turns any panic into "damaged archive".
- **No freeze on a share that stops answering.** A path can belong to a kernel, sshfs or FUSE mount of a share, where every `stat` and `read` blocks for as long as the server is gone. Properties used to look at the file on the main thread to decide its Shortcut tab and whether a folder is a link, and so did opening a `.desktop` link, the startup folder check, adding a gallery folder and a failed copy's drive name. These now ask a worker thread and wait at most 400 ms. A local file answers at once. If a file doesn't answer, the window carries on without that fact: Properties opens without its Shortcut, Sharing and Customize tabs, and a link opens as a plain file.
- **Launchers:** a pipe named `something.desktop` no longer hangs the window, in Properties or on open. Launchers over 1 MiB are not read.
- **Safety net:**
  - Panics are written with a backtrace to `~/.cache/winspace/crash.log`.
  - The app notices at the next start when the last run ended abnormally.
  - **Reopen my tabs** still brings the tabs back after a crash and shows nothing about it. The tabs are saved every 3 seconds.
  - If the last two runs both died within their first minute, the restore is skipped once, quietly, so a tab that crashes the app cannot crash it at every start.

Media playback in the Preview pane is not changed here; it is moving out of process separately.

## Cost

Starting the decoder takes about 25 ms per picture on the test machine (bubblewrap adds almost nothing), spread over the thumbnail slots.

## Testing

- Tests written first. The 7z one panics on the old code and passes with the fix.
- The decoder child, tested against:
  - a child that aborts or hangs, and one cancelled mid-decode;
  - endless input;
  - 2,000 mangled answers;
  - a corpus of broken pictures generated in the test: cut-short files, huge and negative sizes, SVG entity bombs, deep nesting, self-references and seeded mutations.
- Seeded mutation fuzzing:
  - 40,000 rounds each of ZIP, 7z and four tar flavors;
  - 600,000 per sample for the RTF reader, the text decoder and the picture header readers.
- Real-input smoke run under a private Xvfb: 300 generated files, a third of them broken, arrowed through with the Preview pane and then the Details pane. The app stayed up, with nothing on stderr matching "Bail out", CRITICAL, "panicked" or a signal.
- Stalled-mount tests: Properties of a launcher or folder on a stalled mount, and of a pipe named `.desktop`, open in under 3 seconds. A test cannot mount a real share, so a stalled mount is simulated by delaying every look under a folder.
- `native/parity/check.py`, rustfmt and Clippy with Rust 1.92.0, and every test executable in `check.py`'s isolation. The container only has root, and the app refuses to run as root, so `check.py` stopped at the first test that starts the app. The 21 ox-app and ox-core tests that failed fail the same way on unchanged main.
