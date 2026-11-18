// SPDX-License-Identifier: AGPL-3.0-only
//! The places and items a Windows icon file can draw, and which icon group
//! of which file draws each.
//!
//! The group numbers are the resource IDs of `imageres.dll.mun` and
//! `shell32.dll.mun` in Windows 10 and 11 (checked on Windows 11 build
//! 26200; Home's and Gallery's are the ones their `DefaultIcon` keys
//! name). A file is recognized by its name alone, with or without the
//! `.mun` of the copies in `C:\Windows\SystemResources`.

use std::path::Path;

/// A Windows file whose icon groups `OpenXplorer` knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconLibrary {
    /// `imageres.dll`, or its resources `imageres.dll.mun`.
    Imageres,
    /// `shell32.dll`, or its resources `shell32.dll.mun`.
    Shell32,
}

impl IconLibrary {
    /// The library `path` names, by its file name in any letter case, with
    /// or without `.mun`; `None` for any other file.
    pub fn of_file(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_ascii_lowercase();
        let name = name.strip_suffix(".mun").unwrap_or(&name);
        match name {
            "imageres.dll" => Some(Self::Imageres),
            "shell32.dll" => Some(Self::Shell32),
            _ => None,
        }
    }
}

/// A place or item `OpenXplorer` can draw with an icon from a Windows file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IconRole {
    /// A folder.
    Folder,
    /// The Desktop folder.
    Desktop,
    /// The Downloads folder.
    Downloads,
    /// The Documents folder.
    Documents,
    /// The Pictures folder.
    Pictures,
    /// The Music folder.
    Music,
    /// The Videos folder.
    Videos,
    /// Home.
    Home,
    /// Gallery.
    Gallery,
    /// This PC.
    ThisPc,
    /// A drive.
    Drive,
    /// The drive the system runs from (Local Disk).
    SystemDrive,
    /// The Recycle Bin with nothing in it.
    RecycleBinEmpty,
    /// The Recycle Bin with items in it.
    RecycleBinFull,
    /// A ZIP archive, shown as a folder.
    ZipFolder,
    /// A text file.
    TextFile,
    /// A file of a type without an icon of its own.
    GenericFile,
}

impl IconRole {
    /// Every role, in the order of [`IconRole`].
    pub const ALL: [IconRole; 17] = [
        IconRole::Folder,
        IconRole::Desktop,
        IconRole::Downloads,
        IconRole::Documents,
        IconRole::Pictures,
        IconRole::Music,
        IconRole::Videos,
        IconRole::Home,
        IconRole::Gallery,
        IconRole::ThisPc,
        IconRole::Drive,
        IconRole::SystemDrive,
        IconRole::RecycleBinEmpty,
        IconRole::RecycleBinFull,
        IconRole::ZipFolder,
        IconRole::TextFile,
        IconRole::GenericFile,
    ];

    /// The file and icon group that draw this role.
    pub const fn source(self) -> (IconLibrary, u16) {
        use IconLibrary::{Imageres, Shell32};
        match self {
            IconRole::Folder => (Imageres, 3),
            IconRole::Desktop => (Imageres, 183),
            IconRole::Downloads => (Imageres, 184),
            IconRole::Documents => (Imageres, 1002),
            IconRole::Pictures => (Imageres, 1003),
            IconRole::Music => (Imageres, 1004),
            IconRole::Videos => (Imageres, 1005),
            IconRole::Home => (Shell32, 51380),
            IconRole::Gallery => (Shell32, 51586),
            IconRole::ThisPc => (Imageres, 109),
            IconRole::Drive => (Imageres, 32),
            IconRole::SystemDrive => (Imageres, 36),
            IconRole::RecycleBinEmpty => (Imageres, 55),
            IconRole::RecycleBinFull => (Imageres, 54),
            IconRole::ZipFolder => (Imageres, 175),
            IconRole::TextFile => (Imageres, 102),
            IconRole::GenericFile => (Imageres, 2),
        }
    }

    /// The role's name in the cache's file names and index.
    pub const fn key(self) -> &'static str {
        match self {
            IconRole::Folder => "folder",
            IconRole::Desktop => "desktop",
            IconRole::Downloads => "downloads",
            IconRole::Documents => "documents",
            IconRole::Pictures => "pictures",
            IconRole::Music => "music",
            IconRole::Videos => "videos",
            IconRole::Home => "home",
            IconRole::Gallery => "gallery",
            IconRole::ThisPc => "this-pc",
            IconRole::Drive => "drive",
            IconRole::SystemDrive => "system-drive",
            IconRole::RecycleBinEmpty => "recycle-bin-empty",
            IconRole::RecycleBinFull => "recycle-bin-full",
            IconRole::ZipFolder => "zip-folder",
            IconRole::TextFile => "text-file",
            IconRole::GenericFile => "generic-file",
        }
    }

    /// The role whose [`Self::key`] is `key`.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|role| role.key() == key)
    }
}
