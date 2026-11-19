# Review 5: crashes, hangs and data safety in OpenXplorer 2.0.20

Upstream main a77936a7 (2.0.20), `cargo build --release --locked` (default application ID `io.winspace.Development.Native`; only the About box differs from the packaged stable ID). Report only: no app code was changed.

## How it was run

- Three unprivileged users (`oxtest`, `oxtest2`, `oxtest3`), each with its own private Xvfb (1600x1000, Openbox), a throwaway HOME and XDG directories, its own `dbus-run-session`, GVfs and a PulseAudio null sink so GStreamer plays through a real audio sink. Never the live desktop.
- A Weston run (x11 backend nested in a private Xvfb, the app on `GDK_BACKEND=wayland`).
- Real input with xdotool. The app ran under `gdb -batch`, so a crash prints a backtrace. A watchdog called `org.gtk.Actions.List` on the app's D-Bus name every 250 ms (GDBus runs it on the GTK main thread) and logged any reply slower than 1 s.
- Local test servers: Samba (guest share `smb://127.0.0.1/Projects`, through GVfs) and sshd plus sshfs (a FUSE mount, standing in for a kernel CIFS/NFS mount; this kernel has no CIFS). Servers were frozen with SIGSTOP and killed to make shares vanish. A 50 MB tmpfs for low disk.
- Environment: Ubuntu 24.04 libraries, GTK 4.14.5, GStreamer 1.24.2, GLib 2.80.0, librsvg 2.58.0, bubblewrap 0.9 (so desktop thumbnailers ran sandboxed). Soft open-file limit 1024 (the usual systemd default for desktop sessions).

Fictional files (script in the session, all generated): mp3, ogg, flac, wav, mp4 (H.264), mkv, webm (VP8) from `audiotestsrc`/`videotestsrc`, 1 s and 5 min; for each long one a truncated half, a copy with 4 KB of random bytes in the middle, one with a random 64-byte header and a 200-byte stub; empty and random "mp4"; PDFs of 1, 5 and 300 pages plus truncated, garbage and header-only; 8000x6000 JPEG, progressive JPEG, 20000x20000 PNG, WEBP, SVGs; odt/docx/xlsx/ods; good and broken `.desktop`; symlink loops, dangling and self links; names 250 bytes long, with a newline, a tab, invalid UTF-8, leading and trailing spaces, emoji and an RTL override; unreadable files and folders; FIFOs named `.mp3`, `.pdf`, `.txt`, `.mp4`, `.jpg`, `.desktop`; empty file; 2 GB sparse files; folders of 10,000 text files and 10,000 mixed media/picture/PDF files; a 200-file folder of mixed media and PDFs; corrupt, truncated, 50,000-entry, 2 GB zip-bomb and `../` traversal archives; a 3 GB and five 200 MB random files.

## Crashes and hangs

### 1. Crash: every audio or video the Preview or Details pane shows can leave its GStreamer player behind, until the app runs out of file descriptors and GLib aborts it

**Steps**
1. Folder with many audio/video files, for example 200 copies of short mp3, ogg, mp4, webm, mkv and flac files, plus some PDFs:
   `gst-launch-1.0 videotestsrc num-buffers=30 ! video/x-raw,width=320,height=240,framerate=30/1 ! x264enc speed-preset=ultrafast ! mp4mux ! filesink location=Clip.mp4` (and the same with `vp8enc ! webmmux`, `audiotestsrc num-buffers=50 ! audioconvert ! lamemp3enc ! id3v2mux`, and so on), then copy each 25 times.
2. Turn on the Preview pane (Alt+P) or the Details pane (Alt+Shift+P). Auto-play on or off makes no difference to the crash.
3. Click the first file and hold Down (or press it 25 times a second), then Up, through the folder.
4. Within about 15 to 20 seconds the window disappears.

**How often**
- Fast arrow keys through the 200-file folder: 7 of 7 crashes (X11: Preview with auto-play 3/3, Preview without auto-play 1/1, Details with and without auto-play 2/2; Wayland in Weston: 1/1). With neither pane: 0 of 1 (control).
- At human speed it still leaks, just slower. Players still alive 20 s after the scroll, and still alive after leaving the folder:

| Run (X11) | Files passed | Players left alive |
|---|---|---|
| 20 short mp4, 1.5 s each, Preview, auto-play | 19 | 11 |
| 20 short mp3, 1.5 s each, Preview, auto-play | 19 | 4 |
| 20 copies each of 12 formats (good and corrupt), 0.3 s each, auto-play off | 20 per format | 2 or 3 per format, every format except a 0-byte file |
| 200 mixed files, 1 s each, auto-play, 17.5 minutes | 1,050 | open files grew from 24 to 156, threads from 23 to 136 |

  Each leftover player holds about 8 threads and 10 open files, so roughly 100 of them reach the 1024 limit. Someone previewing videos one by one with auto-play on would hit it after about 200 videos; with fast keyboard browsing it takes seconds. In the Wayland run at 1.5 s per file only the current player stayed alive (1 of 19), but fast scrolling crashed it the same way.

**stderr** (every crash)
```
(io.winspace.Development.Native:7502): GStreamer-CRITICAL **: gst_poll_write_control: assertion 'set != NULL' failed
(io.winspace.Development.Native:7502): GStreamer-CRITICAL **: gst_poll_read_control: assertion 'set != NULL' failed
(io.winspace.Development.Native:7502): GLib-ERROR **: 00:50:03.427: Creating pipes for GWakeup: Too many open files
Thread 561 "pool-io.winspac" received signal SIGTRAP, Trace/breakpoint trap.
```
**Backtrace** of the thread that died (the victim is whichever call next needs a pipe; here GVfs D-Bus from a GIO worker):
```
#0  g_log_structured_array () libglib-2.0.so.0
#3  g_log ()
#5  g_main_context_new_with_flags ()
#6  g_dbus_connection_send_message_with_reply_sync () libgio-2.0.so.0
#9  g_dbus_proxy_call_sync ()
#10 ??? () gio/modules/libgvfsdbus.so
```
At the crash there were 869 threads, among them 124 `GtkGstPlay`, 124 `typefind:sink`, 63 `vqueue:src`, 61 `aqueue:src` and 4 x 44 `av:h264` decoder threads. The leftover `GtkGstPlay` threads sit idle in `g_main_loop_run` inside `libmedia-gstreamer.so`.

**Suspected code**
- `native/crates/ox-app/src/window/details_pane/media.rs`, `show_player` (about line 242): a new `gtk::MediaFile::for_filename(path)` is made for every selected audio or video file, wrapped in a `gtk::Video` or `gtk::MediaControls`, and dropped through `set_preview_widget` when the selection moves on. `load_player` starts this for each selection after one `spawn_blocking`, with no delay.
- The leak itself is in GTK 4.14's GStreamer media backend: a 60-line C program that only creates a `GtkMediaFile`, puts it in a `GtkVideo` and replaces it every 40 or 300 ms leaks 1 to 6 `GtkGstPlay` per 40 replacements. Calling `gtk_media_file_clear()` or `gtk_media_stream_pause()` on the old stream first did not help (3 runs each). So the app cannot fix it by tidying up; it has to make fewer players or keep them out of its process.

**Suggested fix**
1. Raise the soft `RLIMIT_NOFILE` to the hard limit at startup (`rustix::process::setrlimit`, already a dependency; the hard limit is usually 524288). This alone turns the crash into a slow memory leak for normal sessions.
2. Make a player only once the selection has stayed on the file for about 300 to 500 ms, and never more than one at a time (cancel a pending one when the selection moves).
3. Better: play out of process, as `media_details.rs` already reads media details in a child process with CPU and memory limits. A child per preview (or one long-lived child) that the pane kills when the selection changes takes every leaked thread and file with it, and a GStreamer abort such as the decodebin3 one the user hit would only end the preview, not the window.
4. Alternatively, keep one long-lived `gstreamer::Pipeline` (playbin3 with a GTK4 paintable sink) and change its URI after setting it to NULL, so nothing is torn down per selection.

### 2. Hang: Properties of a `.desktop` file reads it on the main thread, so a slow share freezes the app for the whole outage, and a named pipe freezes it for good

**Steps (network share)**
1. A folder on a network mount (here sshfs: `sshfs -p 2222 oxtest3@127.0.0.1:/home/oxtest3/remote-data ~/nas`) holding an ordinary launcher, `Apps/Launcher.desktop`.
2. Open `~/nas/Apps`, select `Launcher`.
3. The server stops answering (here `kill -STOP` on `sftp-server`; on a real NAS: Wi-Fi drops, the NAS sleeps or reboots).
4. Press Alt+Enter. The window freezes and stays frozen until the server answers again: 20 s in a run where the server came back after 20 s, about 2 minutes in a run where I left it frozen. Main thread in `openat`, kernel wait channel `request_wait_answer`.

**Steps (named pipe)**
1. `mkfifo ~/Work/Fifo/Launcher.desktop`
2. Open that folder, select `Launcher` (Type: Pipe; the Details pane shows it fine), press Alt+Enter (or right-click, Properties).
3. No Properties window appears and the whole app stops responding. It stays that way until something writes to the pipe or the process is killed (wait channel `wait_for_partner`).

**How often**: share 2 of 2, pipe 1 of 1. It follows from the code (deterministic). Properties of other files and folders on the same frozen mount did not block.

**Evidence**: the watchdog got no reply for 30 s and kept getting none. Backtrace after SIGINT (the share run shows the same frames with `/home/oxtest3/nas/Apps/Launcher.desktop`):
```
#0  __libc_open64 (file="/home/oxtest3/Work/Fifo/Launcher.desktop", oflag=O_RDONLY|O_CLOEXEC)
#1  g_key_file_load_from_file ()
#2  glib::auto::key_file::KeyFile::load_from_file
#3  ox_app::properties::shortcut_tab::read_launcher
#4  ox_app::properties::shortcut_tab::ShortcutKind::of
#5  ox_app::properties::item_window::open_item_properties
#6  ox_app::window::BrowserWindow::show_mounted_properties
#7  ox_app::window::BrowserWindow::show_properties
#8  ox_app::window::BrowserWindow::open_properties
```

**Suspected code**: `native/crates/ox-app/src/properties/shortcut_tab.rs`, `ShortcutKind::of` and `read_launcher` (about lines 51-85; `launcher_facts` near line 276 reads it again). It checks only the extension, then calls `KeyFile::load_from_file` on the main thread, before the Properties window exists.

**Suggested fix**: before reading, `symlink_metadata` and require `is_file()` and a small size limit (as `window/desktop_link.rs` already does with 64 KiB), open with `O_NONBLOCK`, and do the read in `gio::spawn_blocking`, filling the Shortcut tab when it returns.

### 3. One more crash is a possible security issue

It was reported to the owner directly and is not described here.

### Not reproduced: the user's decodebin3 crash on renaming a playing file

`mq_slot_handle_stream_start: assertion failed: (collection)` did not happen in any of 102 renames of a file playing in a pane (plus 21 with no pane):
- 42 in the action matrix below (F2, right-click Rename and the Properties name box; 7 formats; Preview and Details pane), and
- 60 extra F2 renames of a playing long mp4, mp3 and mkv with Preview and auto-play, waiting 1 to 4 s before renaming.

It may need the user's own files or a GStreamer 1.26 decoder. Crash 1's out-of-process fix would contain it too.

## Gtk-CRITICALs and warnings

Counted over all 452 runs (app lines only):

| Count | Message | When |
|---|---|---|
| 86, 59, 15, 6 | `GStreamer-CRITICAL: gst_poll_write_control / gst_poll_read_control / gst_poll_get_read_gpollfd / gst_poll_free: assertion 'set != NULL' failed` | Only in the seconds before crash 1, as players fail to get pipes |
| 3, 3, 3 | `GStreamer-CRITICAL: gst_bus_create_watch: assertion 'bus->priv->poll != NULL'`, `GLib-CRITICAL: g_source_attach / g_source_set_callback: assertion 'source != NULL'` | Same |
| 1 | `GLib-GIO-WARNING: Error creating IO channel for /proc/self/mountinfo: Too many open files` | Same |
| 42 | `Gtk-WARNING: GtkText - did not receive a focus-out event.` | After an inline rename (F2 or right-click Rename) is confirmed, mostly while media is playing in the pane (15 of them in the 30-minute session); harmless so far |
| 1 per try | `GLib-CRITICAL: g_close(fd:28) failed with EBADF. The tracking of file descriptors got messed up` | 40 ms after the sshfs server is killed during a copy to it (2 of 2). GLib's own bug: `gio copy` prints the same line in the same case. The copy then shows "Interrupted Action" correctly. A double close can close a file another thread just opened, so worth an upstream GLib report |
| 22 per run | `GLib-GObject-CRITICAL: g_signal_connect_data: assertion 'G_TYPE_CHECK_INSTANCE (instance)' failed` from `(process:N)` | Printed by `gvfsd-trash`, not by OpenXplorer (GVfs 1.54 here) |

No Rust panic and no `Bail out` anywhere.

## Slow

- Fast arrow keys through media with either pane: the main loop stalled 1.6 to 2.5 s just before each crash 1 (6 of 6 X11 runs).
- One 2.8 s stall with the Details pane while a playing mp4 was replaced on disk five times in two seconds (`mv -f` of other files over it). 1 of 7 tries; the other 6 were clean, and the one stall happened while three sessions shared four CPUs, so it may be load.
- Nothing else passed 1 s: 10,000-file folders in Details and Large icons (scrolled end to end), search over about 20,000 files while a script created, deleted and renamed files, 60 PDFs of 300 pages plus 48 MP photos and 50 MB one-line text files at 25 files a second, Properties opened and closed on 99 files of every type, a frozen SMB server and a frozen sshfs mount (apart from hang 2).

## What held up (no crash, hang or data problem)

- **Matrix**: on a playing or previewed file, F2 rename, right-click Rename, Properties name box, Delete (to the Recycle Bin, checked there), Shift+Delete (confirmed), cut and paste to another folder, paste with Replace (new contents checked by size), and outside changes while it plays (append, truncate to 1000 bytes and to 0, atomic replace, rm, mv, chmod 000 on the file and on its folder, five swaps of different formats). 16 actions x 7 formats (mp3, mp4, mkv, ogg, webm, flac, wav) with the Preview pane (112) and the Details pane (112), and neither pane (112): 336 runs. Every case ended with the expected files, and no run crashed.
- Odd files: FIFOs named `.mp3`/`.mp4`/`.pdf`/`.txt`/`.jpg` selected with each pane; unreadable files and folders; invalid UTF-8, newline, tab and 250-byte names; symlink loops (search and browse); 2 GB sparse files.
- Archives: the 2 GB zip bomb is refused by the per-file safety limit; `../` and absolute paths are refused with nothing extracted; a 50,000-entry zip into a 50 MB tmpfs stops with "No space left on device" and removes its partial output; corrupt and truncated archives open with an error.
- Low disk: copying 465 MB into a 50 MB tmpfs is refused up front ("There is not enough space").
- Copies: two 3.8 GB copies at once (both verified by md5); cancel at 26% to an sshfs folder leaves nothing; killing the server mid-copy shows "Interrupted Action" and "This location is unavailable", sources unchanged; closing the window during a copy asks "A file operation is running" and exits in 127 ms after "Cancel and close".
- Undo/redo: 8 renames, a cut-and-paste of 5 files and a Recycle Bin delete of 3, then 25 x Ctrl+Z, 25 x Ctrl+Y, 25 x Ctrl+Z and 10 Ctrl+Y/Ctrl+Z pairs as fast as xdotool types: all 30 files' contents still present (md5).
- Tabs: 25 opened then closed, 30 open/close pairs, 8 tabs closed while a 10,000-file folder loaded; drag and drop of a file onto another tab moves it there.
- Network: SMB server frozen (SIGSTOP) while browsing, refreshing, going up and opening a second tab: the UI stayed responsive ("Working on it...") and Ctrl+Q quit in 1 s. sshfs frozen: selection, Properties of a folder, refresh, typing a path: no stall in 2 minutes (Properties of a `.desktop` file there is hang 2). sshfs killed: "This location is unavailable".
- 30-minute mixed session (Details pane on, seeded random actions about every 3 s: go to one of 8 folders, arrow keys, toggle both panes, change view, search, Properties, new folder plus rename plus delete plus undo, copy and paste from Docs, open and close tabs, refresh, undo/redo, while a script created and deleted files in the work folder every second): 633 actions, no crash, no stall over 1 s, the source folders unchanged by md5. Open files grew from 28 to 71 and threads from 23 to 54 over the 30 minutes: crash 1's leak at a slow pace.

## Leads from reading the code, not reproduced

The same pattern as hang 2: a check decides "local" by `gio::File::path()` being `Some`, which is also true for kernel CIFS/NFS mounts and GVfs FUSE paths, then does file I/O on the main thread. On the sshfs mount these did not block, probably because FUSE answered from its attribute cache; a hard NFS mount or an uncached CIFS path may not.
- `ox-app/src/application/state.rs` about lines 402-410 (`startup_location`, from `open_first_tab` and `open_split`): `path.is_dir()` for the startup folder when a window opens, even when an explicit folder is passed.
- `ox-app/src/properties/item_window.rs` about lines 317-326: `symlink_metadata` before a folder's Properties window opens.
- `ox-app/src/window/desktop_link.rs` about lines 30-36: `metadata` and `read_to_string` of a `.desktop` link opened from a share, in a main-loop future.
- `ox-app/src/window/file_ops/failure_dialog.rs` about line 301: `find_enclosing_mount` (synchronous) when a copy to a network folder fails. With a killed sshfs server the dialog came up at once; with a hung server it was not tried.

## Not tested

A real NFS or kernel CIFS mount (no CIFS in this kernel; sshfs stood in), HiDPI, a real GPU, PipeWire (PulseAudio null sink instead), the user's own media files, a disk filling up during a copy rather than before it (the up-front space check always caught it), the packaged stable build.
