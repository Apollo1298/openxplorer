// SPDX-License-Identifier: AGPL-3.0-only
//! Tests of icons from Windows icon files, on synthetic files the tests
//! build ([`ox_core::windows_icons::synthetic`]): their pictures are test
//! patterns, never Windows icons.

use std::path::{Path, PathBuf};

use gtk::prelude::*;
use ox_core::places::KnownFolder;
use ox_core::settings::Preferences;
use ox_core::windows_icons::synthetic::SyntheticIconFile;
use ox_core::windows_icons::{IconLibrary, IconRole};

use super::*;
use crate::icons::art::{NetworkArt, NetworkPlace};
use crate::icons::{Connection, Medium};

/// Turns the icons off again when a test ends, also when it fails, so the
/// next test starts with the app's own icons.
struct Reset;

impl Drop for Reset {
    fn drop(&mut self) {
        follow_preferences(&Preferences::default());
    }
}

/// Writes synthetic `imageres.dll.mun` and `shell32.dll.mun` files with
/// every group the roles use into `folder`.
fn write_both_libraries(folder: &Path) -> Vec<PathBuf> {
    [
        ("imageres.dll.mun", IconLibrary::Imageres),
        ("shell32.dll.mun", IconLibrary::Shell32),
    ]
    .into_iter()
    .map(|(name, library)| {
        let mut groups: Vec<u16> = IconRole::ALL
            .into_iter()
            .map(IconRole::source)
            .filter(|(source, _, _)| *source == library)
            .map(|(_, group, _)| group)
            .collect();
        groups.sort_unstable();
        groups.dedup();
        let path = folder.join(name);
        std::fs::write(&path, SyntheticIconFile::with_groups(&groups).build()).expect("written");
        path
    })
    .collect()
}

/// Preferences with the icons on and `files` chosen.
fn chosen(files: &[PathBuf]) -> Preferences {
    Preferences {
        use_icon_files: true,
        icon_files: files.iter().map(|path| path.display().to_string()).collect(),
        ..Preferences::default()
    }
}

/// The edge of `role`'s texture for `size` at `scale`.
fn texture_edge(role: IconRole, size: i32, scale: i32) -> Option<i32> {
    texture_of(role, size, scale).map(|texture| texture.width())
}

/// A network location on the bar, as the art of a sidebar row shows it.
fn network(place: NetworkPlace, connection: Connection) -> Art {
    Art::Network(NetworkArt { place, connection })
}

/// Every role has the art that shows it, wherever that art is drawn: the
/// standard folders as places, as listed items and on the network; the
/// drives by their medium; the network's places; the file kinds; and the
/// Settings tab.
///
/// parity: LOOK-029
#[test]
fn every_role_has_the_art_that_shows_it() {
    use Connection::{Connected, Disconnected};
    let cases = [
        (Art::Folder, IconRole::Folder),
        (Art::ZipFolder, IconRole::ZipFolder),
        (Art::File(FileType::Archive), IconRole::ZipFolder),
        (Art::File(FileType::Text), IconRole::TextFile),
        (Art::File(FileType::Image), IconRole::ImageFile),
        (Art::File(FileType::Audio), IconRole::AudioFile),
        (Art::File(FileType::Video), IconRole::VideoFile),
        (Art::File(FileType::Executable), IconRole::Executable),
        (Art::File(FileType::Script), IconRole::Script),
        (Art::File(FileType::Shortcut), IconRole::Shortcut),
        (Art::File(FileType::Document), IconRole::GenericFile),
        (Art::File(FileType::Pdf), IconRole::GenericFile),
        (Art::File(FileType::Spreadsheet), IconRole::GenericFile),
        (Art::File(FileType::Code), IconRole::GenericFile),
        (Art::Glyph(Icon::HardDrive), IconRole::Drive),
        (Art::RemovableDrive(Medium::Disk), IconRole::RemovableDrive),
        (Art::RemovableDrive(Medium::Optical), IconRole::OpticalDrive),
        (Art::Glyph(Icon::Delete), IconRole::RecycleBinEmpty),
        (
            Art::TintedGlyph(Icon::Delete, Tint::Home),
            IconRole::RecycleBinFull,
        ),
        (Art::TintedGlyph(Icon::Home, Tint::Home), IconRole::Home),
        (Art::Glyph(Icon::Home), IconRole::Home),
        (Art::TintedGlyph(Icon::Image, Tint::Home), IconRole::Gallery),
        (Art::TintedGlyph(Icon::Laptop, Tint::ThisPc), IconRole::ThisPc),
        (Art::Glyph(Icon::Laptop), IconRole::ThisPc),
        (
            Art::TintedGlyph(Icon::Organization, Tint::Network),
            IconRole::Network,
        ),
        (Art::Glyph(Icon::Organization), IconRole::Network),
        (
            network(NetworkPlace::Server, Connected),
            IconRole::NetworkComputer,
        ),
        (network(NetworkPlace::Share, Connected), IconRole::NetworkShare),
        (
            network(NetworkPlace::MappedDrive, Connected),
            IconRole::NetworkDrive,
        ),
        (
            network(NetworkPlace::MappedDrive, Disconnected),
            IconRole::NetworkDriveDisconnected,
        ),
        (
            network(NetworkPlace::KnownFolder(KnownFolder::Music), Connected),
            IconRole::Music,
        ),
        (Art::Glyph(Icon::Settings), IconRole::Settings),
        (Art::StandardFolder(KnownFolder::Templates), IconRole::Folder),
    ];
    for (art, role) in cases {
        assert_eq!(role_of(art), Some(role), "{art:?}");
    }
    let folders = [
        (KnownFolder::Desktop, IconRole::Desktop),
        (KnownFolder::Downloads, IconRole::Downloads),
        (KnownFolder::Documents, IconRole::Documents),
        (KnownFolder::Pictures, IconRole::Pictures),
        (KnownFolder::Music, IconRole::Music),
        (KnownFolder::Videos, IconRole::Videos),
    ];
    for (folder, role) in folders {
        assert_eq!(
            role_of(Art::for_known_folder(folder)),
            Some(role),
            "{folder:?} as a place"
        );
        assert_eq!(
            role_of(Art::StandardFolder(folder)),
            Some(role),
            "{folder:?} as an item"
        );
    }
    // Every role is reached by some art.
    let reached: Vec<IconRole> = cases
        .iter()
        .chain(
            folders
                .iter()
                .map(|(folder, role)| (Art::StandardFolder(*folder), *role))
                .collect::<Vec<_>>()
                .iter(),
        )
        .map(|(_, role)| *role)
        .collect();
    for role in IconRole::ALL {
        assert!(reached.contains(&role), "{role:?} is drawn somewhere");
    }
}

/// A disconnected share keeps the app's red cross, and the glyphs no
/// Windows icon file draws keep the app's own.
///
/// parity: LOOK-029
#[test]
fn other_art_keeps_the_app_icons() {
    use Connection::Disconnected;
    let others = [
        network(NetworkPlace::Share, Disconnected),
        Art::Glyph(Icon::History),
        Art::Glyph(Icon::Clock),
        Art::Glyph(Icon::Phone),
        Art::Glyph(Icon::Image),
        Art::for_known_folder(KnownFolder::Templates),
    ];
    for art in others {
        assert_eq!(role_of(art), None, "{art:?} keeps the app's own icon");
    }
}

/// Off by default: no texture for any role, and an image shows the app's
/// own icon.
///
/// parity: LOOK-029
#[gtk::test]
fn the_icons_are_off_by_default() {
    let _reset = Reset;
    follow_preferences(&Preferences::default());
    assert_eq!(status(), Status::Off);
    for role in IconRole::ALL {
        assert!(texture_of(role, 16, 1).is_none(), "{role:?}");
    }
    let image = ArtImage::new(Art::Folder, 16);
    assert!(!image.shows_user_icon());
    assert_eq!(
        image.picture().icon_name().as_deref(),
        Some(Icon::FileFolder.name())
    );
}

/// Once on, the chosen files draw every role at the size that fits each
/// place and scale, images already shown follow, and turning it off brings
/// the app's own icons back.
///
/// parity: LOOK-029
#[gtk::test]
fn chosen_files_draw_their_roles_at_the_best_size() {
    let _reset = Reset;
    let folder = tempfile::tempdir().expect("a temporary folder");
    let files = write_both_libraries(folder.path());
    let folder_image = ArtImage::new(Art::Folder, 16);
    let zip = ArtImage::new(Art::ZipFolder, 48);
    let share = ArtImage::new(
        Art::Network(NetworkArt {
            place: NetworkPlace::Share,
            connection: Connection::Disconnected,
        }),
        16,
    );

    follow_preferences(&chosen(&files));
    wait_until_read();
    assert_eq!(
        status(),
        Status::Ready {
            found: IconRole::ALL.len()
        }
    );
    // The navigation pane's 16 pixels, at 1x and 2x; the lists' 32 and 48;
    // a large view's 96 and 256.
    assert_eq!(texture_edge(IconRole::Home, 16, 1), Some(16));
    assert_eq!(texture_edge(IconRole::Home, 16, 2), Some(32));
    assert_eq!(texture_edge(IconRole::Folder, 20, 1), Some(32));
    assert_eq!(texture_edge(IconRole::Folder, 32, 1), Some(32));
    assert_eq!(texture_edge(IconRole::Folder, 48, 1), Some(256));
    assert_eq!(texture_edge(IconRole::RecycleBinFull, 256, 1), Some(256));

    assert!(folder_image.shows_user_icon(), "a shown image follows");
    let picture = folder_image.picture();
    assert_eq!(picture.pixel_size(), 16);
    assert!(picture.paintable().is_some());
    assert!(zip.shows_user_icon());
    let badge = zip.badge_pieces_if_made();
    assert!(
        badge.is_none_or(|visible| !visible),
        "the file's own ZIP icon has no badge"
    );
    assert!(
        !share.shows_user_icon(),
        "a disconnected share keeps the app's red cross"
    );
    let new_image = ArtImage::new(Art::Glyph(Icon::HardDrive), 16);
    assert!(new_image.shows_user_icon(), "a new image starts with it");

    follow_preferences(&Preferences {
        use_icon_files: false,
        ..chosen(&files)
    });
    assert_eq!(status(), Status::Off);
    assert!(!folder_image.shows_user_icon(), "back to the app's own icon");
    assert_eq!(
        folder_image.picture().icon_name().as_deref(),
        Some(Icon::FileFolder.name())
    );
    assert!(texture_of(IconRole::Folder, 16, 1).is_none());
}

/// Files that are damaged, of another name or missing give nothing, and
/// every image keeps the app's own icon; nothing is written beside them,
/// only in the user's cache folder.
///
/// parity: LOOK-029
#[gtk::test]
fn files_that_give_nothing_keep_the_app_icons() {
    let _reset = Reset;
    let folder = tempfile::tempdir().expect("a temporary folder");
    let damaged = folder.path().join("imageres.dll.mun");
    std::fs::write(&damaged, b"MZ, and nothing a PE file needs").expect("written");
    let other = folder.path().join("notepad.exe");
    std::fs::write(&other, SyntheticIconFile::with_groups(&[3]).build()).expect("written");
    let missing = folder.path().join("shell32.dll.mun");

    follow_preferences(&chosen(&[damaged, other, missing]));
    wait_until_read();
    assert_eq!(status(), Status::Ready { found: 0 });
    let image = ArtImage::new(Art::Folder, 32);
    assert!(!image.shows_user_icon());
    assert_eq!(
        image.picture().icon_name().as_deref(),
        Some(Icon::FileFolder.name())
    );

    let beside: Vec<_> = std::fs::read_dir(folder.path())
        .expect("readable")
        .map(|entry| entry.expect("an entry").file_name())
        .collect();
    assert_eq!(beside.len(), 2, "only the two files written: {beside:?}");
    let cache = ox_core::windows_icons::cache_folder();
    assert!(cache.starts_with(glib::user_cache_dir()));
}

/// A plain role image, such as a drive's tile, follows the choice too.
///
/// parity: LOOK-029
#[gtk::test]
fn a_role_image_shows_the_file_icon_or_its_glyph() {
    let _reset = Reset;
    let folder = tempfile::tempdir().expect("a temporary folder");
    let files = write_both_libraries(folder.path());
    let tile = role_image(IconRole::RemovableDrive, Icon::HardDrive, 48);
    assert_eq!(tile.icon_name().as_deref(), Some(Icon::HardDrive.name()));

    follow_preferences(&chosen(&files));
    wait_until_read();
    assert!(tile.icon_name().is_none());
    assert_eq!(
        tile.paintable().map(|paintable| paintable.intrinsic_width()),
        Some(256)
    );

    follow_preferences(&Preferences::default());
    assert_eq!(tile.icon_name().as_deref(), Some(Icon::HardDrive.name()));
}
