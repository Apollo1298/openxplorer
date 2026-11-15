# Title

feat(previews): Show icons instead of thumbnails in network folders by default, as Windows 11 does

# Body

Model: claude-opus-5-5

Network folders now show file-type icons by default, the way Windows 11 File Explorer does. A setting turns thumbnails on for anyone who wants them, and with it on, pictures on SMB shares now get their thumbnails reliably.

Microsoft documents the Windows default: "File Explorer displays only icons and never displays thumbnail images on network folders by default."
https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-admx-windowsexplorer

## What changes

- **Default:** files on SMB shares, other servers (SFTP, FTP, WebDAV, NFS), phones and cameras, and network mounts show their type icons in every view, and the Preview pane says "No preview available." for them. Folders on this computer keep their thumbnails.
- **The setting:** Settings > Files & folders > Previews and thumbnails > **Show thumbnails in network folders**, off by default. Its info text reads: "Off by default, as in Windows: files on shares, servers, phones and cameras show their type icons. Turn on to show their thumbnails and previews too." It is stored only once changed, and open windows follow it at once, without a restart.
- **With the setting on:** network files get thumbnails and Preview pane previews. GVfs's SMB and SFTP backends report only a fast content type for some files (PNG pictures among them), so those files are now typed by that fast type or by their name, and their thumbnails appear. Network thumbnails are made on two slots of their own, separate from the ones for files on this computer.

| | Before | After |
|---|---|---|
| Network folder, default | Thumbnails | Type icons, as in Windows 11 |
| Preview pane on a network file, default | Preview | "No preview available." |
| Local folders | Thumbnails | Thumbnails (unchanged) |
| PNG on an SMB or SFTP share, setting on | Type icon | Thumbnail |
| Setting name | Also in network folders (on) | Show thumbnails in network folders (off) |

Pictures: `network-thumbnails-<scale>-<theme>-share-icons.png` (default), `-share-thumbnails.png` (setting on) and `-settings-row.png`, at 1x and 2x, light and dark.

After review: folder listings now ask GIO for `standard::fast-content-type` too, so a network file that its backend types only by its fast type is typed by it, and with the setting on it gets its thumbnail.

## CHANGED OWNER DECISIONS

- #327 turned thumbnails in network folders on by default. This sets the default back to off, matching Windows 11; turning the setting on gives the #327 behavior.
- The Settings row "Also in network folders" is renamed "Show thumbnails in network folders", with new info text. Its Chinese (zh_CN) translation needs redoing.

## How it was tested

- Tests written first, failing on main: `network_previews_are_off_by_default_as_in_windows` and `a_network_file_without_a_content_type_is_typed_by_its_name` (ox-core), `previews_follow_the_settings`, `the_network_thumbnails_switch_is_off_by_default_and_saved_only_when_changed` and `a_share_that_does_not_answer_leaves_local_thumbnails_their_slots` (ox-app). All pass now.
- `python3 native/parity/check.py`, the i18n template check, `tools/sync-docs.py --check`, rustfmt and clippy with `-D warnings` pass. Every test executable and the doctests ran under Xvfb with a private D-Bus session and disposable home, as `native/tools/check.py` runs them; the only failures are the same tests that fail on unchanged main in this environment (two Python fixture comparisons and nine `admin_copy` tests), plus one rename-in-place test that failed once while the machine was busy and passes on every rerun (three alone, and its whole module).
- The app under Xvfb with real mouse and keyboard input, 1x and 2x, light and dark, against a local Samba share and a local SFTP server holding fictional files: icons by default in Large icons, Tiles and Details; Preview pane "No preview available."; local Pictures thumbnails; turning the setting on and off while the share was open updated the view at once; with it on, PNG, JPG, PDF and MP4 thumbnails and the Preview pane worked on SMB and SFTP; while a share answered slowly, a local folder in another tab still got its thumbnails. No GTK criticals.
- Not tested: a kernel CIFS mount (not available in the test machine's kernel) and a real phone or camera; those follow the same setting through the existing location checks covered by unit tests.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
