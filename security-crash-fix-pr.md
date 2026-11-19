# Title

fix(pictures): Draw SVG pictures in a limited child process so a costly one can't crash or stall the app

# Body

Model: claude-opus-5-5

## Summary

Some SVG files cost far more to draw than their size suggests. A single line of very long text is enough: a 1 MB file made the app allocate gigabytes and abort, and a 100 KB one kept a worker busy for over a minute. The app drew SVGs itself, for thumbnails, the Preview and Details panes, Properties and Gallery, and gdk-pixbuf's SVG loader draws only once its loader is closed, so the existing time limit could not stop it.

Vector pictures are now drawn in a child process, as the Details tab already reads media details (`media_details.rs`):

- `picture_guard::decode_to_fit` (and `decode_reader_to_fit`) look at a picture's first 4 KB. Markup, UTF-16 text or an SVG signature goes to the child; every other format is decoded in the app as before, through the same header checks.
- The child is the app itself started with `--ox-draw-vector-picture WIDTH HEIGHT`. It limits its own processor time (10 s) and memory (1 GiB of data), reads the document on its input, draws it to fit through the same `picture_guard` code and writes back the picture's own size and a PNG.
- The app waits at most the picture's time limit and kills the child then, or when the caller cancels. A child that ran out of time or memory, crashed or answered nonsense leaves the picture refused, so the item shows its type icon. The PNG it writes back is decoded through the raster checks.
- Files over 32 MB are refused without being drawn.

Raster formats, the existing limits and every caller are unchanged. Starting the child costs about 20 to 30 ms per SVG (release build); thumbnails are cached once made.

## How it was tested

- New tests, which failed before the fix:
  - `picture_guard::tests::an_svg_too_large_to_draw_whole_is_drawn_in_part_or_refused`: before, the test process aborted ("memory allocation of 2147483648 bytes failed").
  - `picture_guard::tests::an_svg_slow_to_draw_is_refused_at_the_time_limit`: before, still drawing after 100 s.
- New unit tests in `picture_guard::vector`: telling vector pictures from their first bytes, and refusing a broken answer.
- The existing SVG test (a broken SVG refused, one declaring 100000 x 50000 drawn at 256 x 128) passes through the child.
- Release build, real input on a private Xvfb as an unprivileged user, folder with a 1 MB long-text SVG, a 100 KB long-text SVG, two ordinary SVGs and a PNG: Large icons, the Preview pane on each, Properties of each, and Gallery. The app's memory peaked at 367 MB, no stall over 1 s, ordinary SVGs and the PNG drawn, the slow SVG shown with its type icon. Before the fix, the same folder made the app grow past 3 GB within a second.
- `python3 native/tools/check.py` with Rust 1.92 (the container's 1.97 Clippy flags `ox-core` code this change doesn't touch): parity and tools unit tests, i18n check, icon guard, rustfmt and `clippy -D warnings` passed. It then stops at `display_scale`, which refuses to run as root here, as on unchanged main.
- Every test executable, under check.py's own isolation but continuing past failures: 70 passed; the same 17 fail on unchanged main a77936a7 with the same 33 failing tests (root, keyring and Python-interop cases), none only on this branch. ox-app 2566 passed / 19 failed against 2561 / 19 on main; ox-core 1291 / 2 on both.
- `python3 native/parity/check.py`: passed.
