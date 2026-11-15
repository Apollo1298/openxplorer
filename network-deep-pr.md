# Title

fix(network): Show real progress, plain names and working Undo for network folders

# Body

Model: claude-opus-5-5

Five things went wrong for someone working with network shares (SMB through GVfs, and SFTP) as they would on Windows. Each one now has a test that fails on main and passes with its fix.

- **Progress of one large file.** Any paste with no name conflict dropped the byte total, so copying one big file showed "0% complete" with no time remaining until the very end. This also happened locally, but it is most visible on a slow share. A paste that runs as one step now keeps the measured total. Pastes in several steps, after Replace or Skip answers, still show only the bytes processed, as before.
- **The progress window spelled out a share.** It said "Copying 1 item from smb://nas/media/ to Documents". It now names the share's top folder as its tab does ("from media to Documents"), and an SFTP server root by its host, without the account.
- **A refused sign-in lost the user name.** After a wrong password, the dialog asked again with the local account's name instead of the one typed. It now keeps what was typed, including `DOMAIN\user`.
- **Undo of a copy onto a share never worked.** Undo sends copies to the Recycle Bin. A share has none, so Undo stopped every time with "Trash is not supported at this location" and only OK. Undo now asks Delete's own question for those items ("Are you sure you want to permanently delete this file?", with the file's details). No keeps the copy and the Undo step. Yes deletes it, and Redo then brings back only what went to the Recycle Bin. The transfer engine itself still never deletes permanently on its own.
- **Saving to a share whose server went away.** Another app's Save dialog said "The folder "/run/user/1000/gvfs/smb-share:server=…/Docs" does not exist." It now says "\\nas\media\Docs is unavailable. Make sure you are connected to the network and the server is turned on, and then try again." and answers nothing. Its other messages also name a share's folder by its network address, not by the FUSE path handed to the caller.

| | Before | After |
|---|---|---|
| One 300 MB file from a slow share | "0% complete", no time remaining, "Items remaining: 1 (66.5 MB processed)" | "8% complete", "Time remaining: About 1 minute and 56 seconds", "Items remaining: 1 (274 MB)" |
| Progress description | "Copying 1 item from smb://nas/media/ to Documents" | "Copying 1 item from media to Documents" |
| Wrong password, asked again | User name reset to the local account | The typed user name stays |
| Ctrl+Z after copying a file to a share | "Interrupted Action … Trash is not supported at this location … OK"; nothing undone | "Delete File / Are you sure you want to permanently delete this file?"; Yes removes the copy, No keeps it and the Undo step |
| Save dialog, server gone | "The folder "/run/user/…/gvfs/smb-share:…/Docs" does not exist." | "\\nas\media\Docs is unavailable. Make sure you are connected to the network and the server is turned on, and then try again." |

## Changed owner decisions

- XFER-014 / OPS-029: Undo never deleted anything permanently. Items that cannot go to the Recycle Bin, as on a share, are now deleted permanently, and only after the user answers Yes to Delete's own confirmation. Everything else still goes to the Recycle Bin. The engine still never falls back to a permanent delete by itself.

## Not confirmed (please check on Windows 11)

- Undo of a copy into a network folder: does Windows ask "Are you sure you want to permanently delete this file?" and delete the copy, as this change does?
- The progress window for a copy from a share's top folder: does it say "from Studio", the share's name?
- The network credentials prompt after a wrong password: does it keep the typed user name, and where is the keyboard focus?
- A Save dialog when the share's server has gone away: Windows' exact message.
- Cancel during a multi-file copy: Windows keeps the files already copied. This app removes what it copied in that run. That is existing behavior and is not changed here.

## How it was tested

- **Fixtures, all local and fictional.**
  - Samba 4.19 on 127.0.0.2: a guest share, a read-only share and two password shares.
  - A small TCP proxy on 127.0.0.3 that adds latency, limits bandwidth and drops connections.
  - OpenSSH SFTP on 127.0.0.4.
  - GVfs with its FUSE export.
- **Setup.** The app ran unprivileged on Xvfb with a private D-Bus bus and a throwaway home folder.
- **Input.** Real xdotool input at 1x (light) and 2x (dark).
- **Save dialog.** Another app's Save was simulated by a script that owns `org.freedesktop.portal.Desktop` on that bus and calls the app's FileChooser backend.
- **Checked and working:**
  - A connection dropped mid-copy gives Interrupted Action. Try Again finishes the copy byte-identical.
  - Cancel leaves no partial files. Read-only shares and permission errors are asked about in the progress window.
  - Replace or Skip works on a share.
  - A second share on the same server opens without a prompt, also from the keyring after a restart.
  - SFTP sign-in works after a wrong password.
  - Startup is immediate with an offline pinned share, and Try again works once the server is back.
  - The window stays responsive on a 20 KB/s share: a new tab listed in 0.65 s while 500 files loaded.
  - Saving and replacing on a share through the Save dialog works.
- **New tests:**
  - `a_paste_of_one_large_file_reports_how_much_it_writes`
  - `a_share_or_server_top_folder_is_named_not_spelled_out`
  - `a_rejected_sign_in_asks_again_with_the_typed_user_name`
  - `undoing_a_copy_deletes_the_agreed_copies_that_have_no_recycle_bin`
  - `undoing_a_copy_without_a_recycle_bin_asks_to_delete_it_permanently`
  - `saving_where_the_server_went_away_says_the_folder_is_unavailable`
  - Capture tests in `window/tests/network_captures.rs`.
- **Checks.** rustfmt, Clippy with the workspace lints (no finding in changed code), `native/parity/check.py` and `native/tools/i18n.py check`. The full test suites, compared with unchanged main by test name: 4,564 passed on this branch and 4,555 on main. The same 29 tests fail on both, for reasons in the test machine (its mounts and its Python version). The app's own 2,380 tests all pass.
- **Not tested.** Kernel CIFS mounts (this machine's kernel has no cifs). Dialogs under Weston (its X11 backend showed no dialog windows, Delete's unchanged question included).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
