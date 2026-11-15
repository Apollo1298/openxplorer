# Title
perf(views): Keep the window responsive in folders of 100,000 items

# Body
Model: claude-opus-5-5

Large folders made the window freeze for seconds at a time. With 100,000 files, Select all, Shift+End, Copy and Cut each froze it for 3 to 4.5 seconds, Group by Type for 3.7 seconds, and a folder changed by another program for about a second. Switching Group by from one key to another also logged a GLib critical ("Too many GWeakRef registered").

What changed:

- **Listing.** A reader thread reads the folder and builds its rows, so the interface thread only takes in finished rows and answers input between batches. Batches are spaced by how long the last one took to take in. Row icons are laid out on the reader thread, so comparing rows after a reload is cheap.
- **No tree unless folders expand.** GTK's tree list model rebuilt a node for every item on each batch. It now exists only while folders may expand in place (a setting that is off by default).
- **No filter when nothing is hidden.** With no search, no search option, no dialog narrowing and no hidden item left out, the filter model has no filter and hands items through without searching its bitset. The listing and reload give it the filter before hidden items arrive, and a tab that moves to a folder without hidden items drops it again.
- **Selection.** The selected items are looked up once per change and shared with the status bar, details pane and commands instead of copied for each. Each item's address facts (share root, snapshot or ZIP path, Recent row) are worked out once per item. The details pane no longer checks every selected item for a preview it only shows for one. Snapshot detection skips decoding an address that cannot name a snapshot.
- **Grouping.** Each item's group is worked out once per grouping instead of twice per comparison. Changing Group by sets a new section sorter at once instead of passing through "no groups", which had headed nearly every item as its own group (the cause of the freeze and the GLib critical).
- **Reload merge.** Neighboring changed items are replaced together, and a reload that changed nothing changes nothing.

## Before and after

Release builds, measured under Xvfb with real input (xdotool). "Freeze" is the longest time the main loop did not answer a `_NET_WM_PING`.

| 100,000 files, local disk | Before | After |
|---|---|---|
| First rows shown | 0.13 s | 0.14 s |
| Folder fully listed | 5.4 s | 2.8 s |
| App CPU time to list it | 5.8 s | 3.9 s |
| Memory after listing | 579 MB | 512 MB |
| Ctrl+A (longest freeze) | 3.21 s | 188 ms |
| Shift+End | 3.47 s | 150 ms |
| Ctrl+A with the details pane open | 3.50 s | 199 ms |
| Ctrl+C with everything selected | 3.88 s | 390 ms |
| Ctrl+X with everything selected | 4.35 s | 524 ms |
| Sort by Date modified | 220 ms | 140 ms |
| Sort by Size | 261 ms | 166 ms |
| Group by Type | 3.68 s | 489 ms |
| Group by Date modified | 570 ms | 333 ms |
| Group by (None) | 692 ms | 377 ms |
| 1,000 files added by another program | 974 ms | 319 ms |
| 1,000 files removed by another program | 1.02 s | 552 ms |
| GLib criticals in the whole run | 30 | 0 |

The same pattern holds for 100,000 mixed files and folders, 50,000 and 10,000 files, 10,000 long names, 100,000 files on tmpfs, at 2x scale and in Compact view. Scrolling (wheel, Page Down, End/Home, dragging the scroll bar) and thumbnails while scrolling fast already stayed under 100 ms and still do.

Still slower than we would like: Copy and Cut of 100,000 items (each address is validated before it reaches the clipboard, about 0.4-0.5 s), changing Group by (0.3-0.5 s, mostly GTK's own sort and redraw), and switching to Tiles (about 0.3 s even for 10,000 files, as before).

## Changed owner decisions

None. Nothing visible changes; the folder tree, filtering, grouping and selection behave as before.

## How it was tested

- New tests for each fix, each failing on the code before it: `no_tree_is_made_while_folders_may_not_expand`, `the_selection_shows_a_tree_only_while_folders_may_expand`, `the_selected_items_are_looked_up_once_per_change`, `each_selected_address_is_read_once`, `kept_address_facts_still_mark_read_only_and_inoperable_items`, `rows_are_built_off_the_listing_thread`, `a_slow_delivery_spaces_out_the_next_one`, `the_items_are_filtered_only_while_something_is_left_out`, `a_folder_without_hidden_items_is_shown_unfiltered`, `neighboring_changes_are_made_in_one_change`, `each_items_group_is_worked_out_once_per_grouping`, `grouping_another_way_never_heads_each_item_on_its_own`, `the_quick_snapshot_test_agrees_with_decoding`.
- `cargo fmt --check`, Clippy with the workspace lints and `python3 native/parity/check.py` pass. `native/tools/check.py` passes its inventory, parity, icon, rustfmt and Clippy steps and then stops at `location_fixture_drift`, which fails the same way on the unchanged code here, so every test binary was also run with the script's own isolation: 4,548 passed and 59 failed, and the same 59 fail on the unchanged code (they compare with the Python app's behavior or need a keyring or the administrator helpers).
- Timings come from release builds under Xvfb at 1x and 2x, in normal and Compact view, with fictional folders of 10,000 to 100,000 files on local disk and tmpfs, driven with xdotool. The app was also started on weston's headless Wayland backend at 1x and 2x and listed 100,000 files with no GTK criticals; xdotool cannot drive Wayland, so no input was timed there.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
