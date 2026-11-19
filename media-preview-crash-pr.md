# Title

fix(details-pane): Play previews in a separate process so no media file can close the app

# Body

Model: claude-opus-5-5
CLA: I agree

## Problem and resulting behavior

On Kubuntu, OpenXplorer closed completely while a file was renamed, and again while the user was just browsing a folder of MP3 files. The journal shows `ERROR:../gst/playback/gstdecodebin3.c:3381:mq_slot_handle_stream_start: assertion failed: (collection)`, then `Bail out!` and SIGABRT of the app.

There are two problems behind this, and both happen in the app's own process:

* **Leaked players.** Every recording the Details or Preview pane showed left its GStreamer player alive (GTK's media backend leaks them), at about 10 open files and 8 threads each. Arrowing through a folder of recordings reaches the soft limit of 1024 open files within seconds; then GLib aborts ("Creating pipes for GWakeup: Too many open files") or GStreamer asserts.
* **Aborts inside GStreamer.** An abort inside GStreamer, like this decodebin3 assertion, takes the whole window down.

The Details and Preview panes played audio and video with `gtk::MediaFile` in the app's own process. GTK 4.22 (Ubuntu 26.04) plays through GStreamer's own `GstPlay`, which in GStreamer 1.28 always builds `playbin3`, and so `decodebin3`. The 1.28.2 source no longer reads `GST_PLAY_USE_PLAYBIN3`, so the app can't fall back to the classic playbin. The assertion is at line 3381 in GStreamer 1.28.1 to 1.28.4. A failed `g_assert` aborts whatever process it runs in, and it can't be caught. Windows never closes Explorer because of a media file.

Now:

* **The recording is decoded in a player process.** This is the app started again with `--ox-play-media`, as the Details tab's reader already is (`media_details.rs`). It plays with the classic `playbin` (`decodebin`, never `decodebin3`) and plays the sound itself. It sends scaled RGBA frames, its position and its state back on its standard output, after a hello line. A `gtk::MediaStream` subclass (`media_player/stream.rs`) shows them, so `gtk::Video` and `gtk::MediaControls` work as before.
* **A player starts only after the selection has stayed on a recording for 400 ms,** and at most one runs at a time, so arrowing through a folder starts none.
* **Whatever GStreamer leaks dies with the player process.**
* **A player that aborts, hangs up or sends something malformed** puts the stream in error. The pane shows the file's icon and keeps working, and the next file plays.
* **The player is limited.** It has a 2 GiB data limit, gets SIGKILL when the app dies, and quits when its input closes. It's told to quit and killed after one second if it doesn't.
* **The player stops before anything changes the file.** It is paused, its pipeline set to NULL, then released. This happens when the selection changes, and before the app renames, moves, deletes or replaces the file. That covers renaming in place, the Rename dialog, the Properties name box, batch rename, and every transfer job: its sources, unless copied, and the namesakes a Replace overwrites. The app waits for the player to exit, at most one second. Renaming or moving a folder stops the players of the files inside it.
* **The app raises its soft limit on open files to the hard limit at startup,** as defense in depth.
* **A folder change never restarts a stream mid-playback.** After the app stops a player for an operation, the file's next player waits for Play, even with auto-play on. A monitor event for the same file still keeps the player as it was.

`details_pane/media.rs` only hands off to the new `details_pane/player.rs`, so this merges cleanly with the open file-icons pull request (checked with `git merge-tree`).

## Validation

* **Tests that fail first:** `window/tests/preview_player.rs` (4 tests).
  * `arrowing_through_200_recordings_leaves_nothing_behind` walks 200 generated recordings with each pane, quickly and then slowly. On `main`, the test process's open files grew from 26 to 1,132 and its threads from 21 to 834, and the test failed. With the fix they come back to where they started.
  * On `main`, rename and delete each failed: the stream stopped 25 to 30 ms *after* the file changed, and the renamed file started playing again. A watcher thread timestamps the file change and the stream's own notify timestamps the stop, so the order is checked exactly.
  * The aborting-player test timed out, because the player can't abort without taking the app with it.
  * All four pass with the fix, along with the new protocol and entry tests and `details_preview.rs`.
* **Attempts at the decodebin3 assertion itself:** these ran in private Xvfb sessions as an unprivileged user, with a null PulseAudio sink and fictional files generated with gst-launch (MP3, Ogg, MP4/H.264+AAC, WebM). Input was real (xdotool).
  * Ubuntu 24.04 (GStreamer 1.24.2, GTK 4.14.5), with GTK forced to playbin3: about 240 random actions while playing, including F2 and right-click rename, delete, external rename, overwrite, truncate, delete and rapid selection. No crash.
  * Ubuntu 26.04 root filesystem (GStreamer 1.28.2, GTK 4.22.4, the same decodebin3 code as the report), unfixed app, both pane modes: about 1,700 random actions over 42 minutes, and 4 × 150 s of rapid selection in a Downloads-like folder. That folder had ID3v2.4 MP3s with APIC covers (JPEG and 1280×720 PNG), VBR Xing headers, lengths of 4 s to 6 min 40 s, and names with emoji and Tamil script. I also played those MP3s directly with playbin3 and playbin, and overwrote 64 playing files in place under playbin3. No crash. **The assertion was not reproduced.**
* **The leak crash, real input:** I held Down and then Up three times through 231 generated recordings (MP4, WebM, MKV, MP3 with a cover, Ogg and FLAC) at 25 key presses a second.
  * Unfixed app on Ubuntu 24.04: open files rose 37 → 1,024 and threads 33 → 893, then the app died with SIGTRAP and "Too many open files".
  * Fixed app, 24.04 and 26.04, both panes: open files stayed at 28-31 and threads at 25-31, with no player while arrowing and one once the selection rests. At 1.5 s per file with auto-play on, through 40 files, at most one player ran, and open files (31) and threads (25-29) stayed flat.
  * The app ran with an open-file limit of 20000 (the hard limit) where the user's shell soft limit was 1024.
* **Fixed app, real input:**
  * The main process maps no GStreamer playback or decoder plugins while previewing; the player maps them.
  * `kill -ABRT` of the player while playing leaves the app running with the file's icon in the pane, and the next file plays.
  * F2 rename while playing works (24.04).
  * On 26.04, right-click Rename and Delete while playing work in both pane modes. The renamed MP4 shows its first frame and Length, waiting for Play. 60 rapid clicks across eight files leave exactly one player process.
* **Checks:**
  * `python3 native/parity/check.py` passed. rustfmt and clippy passed with the workspace lints on Rust 1.92.0, the toolchain CI pins. Rust 1.97 reports `duration_suboptimal_units` in five places in `ox-core`, which this change doesn't touch.
  * `python3 native/tools/check.py` did not pass as a whole in this container, because of the environment:
    * As root, the app refuses to start, so `display_scale` fails.
    * With the container's Python 3.13, which can't import the distro's PyGObject, `location_fixture_drift` and `network_python` fail.
    * check.py stops at its first failing executable, so I ran every test executable and the doctests with check.py's own isolation (private Xvfb, D-Bus and home), without stopping, as an unprivileged user, with system Python 3.12 and node 22. 169 of 171 executables passed, with 4,902 tests passed and 10 failed:
      * `ox_app`: 2,588 passed, 1 failed, `a_pinned_folder_menu_copies_compresses_and_creates_in_that_folder`. It passes run alone, and an earlier status note records it timing out in long runs.
      * `ox_core`: 1,284 passed, 9 `admin_copy` tests failed, because they need paths only root can change. Run as root, 12 of those 13 tests pass. `an_fstab_that_cannot_be_read_refuses` fails as root, because root can read the unreadable file. `ox-core` is unchanged from `main`.

## Compatibility and release impact

* **New behavior:** the player is a new command-line mode of the same binary (`--ox-play-media`, next to `--ox-read-media-details`).
* **Dependencies:** none new. The `stdio` feature of the existing `rustix` dependency is enabled, which does not change `Cargo.lock`.
* **No changes to:** persisted data, desktop integration, translations or license notices.
* **Behavior changes:**
  * A file GStreamer can't play now shows its icon instead of an empty player.
  * After a rename, move or replace of the playing file, the file's next player doesn't auto-play.
  * A file replaced in place keeps its icon until it is selected again.
* **Fixtures:** all test and reproduction files are generated, fictional media.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01CQbN3PshAViEgjzDBGj4sP
