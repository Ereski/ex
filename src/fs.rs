// TODO: impl Debug to include extra info as well?

use crate::io::{Error, Result};
use crate::Wrapper;

use std::fmt::Arguments;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct DirBuilder(::std::fs::DirBuilder);

impl DirBuilder {
    pub fn new() -> Self {
        DirBuilder(::std::fs::DirBuilder::new())
    }

    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.0.recursive(recursive);

        self
    }

    pub fn create<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        self.0.create(path).map_err(|x| {
            Error::Filesystem(path.to_owned(), "creating directory", x)
        })
    }
}

impl Deref for DirBuilder {
    type Target = ::std::fs::DirBuilder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DirBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Wrapper<::std::fs::DirBuilder> for DirBuilder {
    fn into_inner(self) -> ::std::fs::DirBuilder {
        self.0
    }
}

pub struct DirEntry(::std::fs::DirEntry);

impl DirEntry {
    fn new(inner: ::std::fs::DirEntry) -> Self {
        DirEntry(inner)
    }

    pub fn metadata(&self) -> Result<Metadata> {
        self.0
            .metadata()
            .map(|x| Metadata::new(x, self.0.path()))
            .map_err(|x| {
                Error::Filesystem(self.0.path(), "getting metadata from", x)
            })
    }

    pub fn file_type(&self) -> Result<FileType> {
        self.0.file_type().map_err(|x| {
            Error::Filesystem(self.0.path(), "getting the file type of", x)
        })
    }
}

impl Deref for DirEntry {
    type Target = ::std::fs::DirEntry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DirEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Wrapper<::std::fs::DirEntry> for DirEntry {
    fn into_inner(self) -> ::std::fs::DirEntry {
        self.0
    }
}

pub struct File {
    inner: ::std::fs::File,
    // TODO: maybe Arc?
    path: PathBuf,
}

impl File {
    fn new(inner: ::std::fs::File, path: PathBuf) -> Self {
        File { inner, path }
    }

    pub fn create<P: AsRef<Path>>(path: P) -> Result<File> {
        let path = path.as_ref().to_owned();
        let inner = ::std::fs::File::create(&path)
            .map_err(|x| Error::Filesystem(path.clone(), "creating file", x))?;

        Ok(Self::new(inner, path))
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let inner = ::std::fs::File::open(&path)
            .map_err(|x| Error::Filesystem(path.clone(), "opening file", x))?;

        Ok(Self::new(inner, path))
    }

    pub fn sync_all(&self) -> Result<()> {
        self.inner
            .sync_all()
            .map_err(|x| Error::Filesystem(self.path.clone(), "synching", x))
    }

    pub fn sync_data(&self) -> Result<()> {
        self.inner.sync_data().map_err(|x| {
            Error::Filesystem(self.path.clone(), "synching data of", x)
        })
    }

    pub fn set_len(&self, size: u64) -> Result<()> {
        self.inner.set_len(size).map_err(|x| {
            Error::Filesystem(self.path.clone(), "setting the length of", x)
        })
    }

    pub fn metadata(&self) -> Result<Metadata> {
        self.inner
            .metadata()
            .map(|x| Metadata::new(x, self.path.clone()))
            .map_err(|x| {
                Error::Filesystem(self.path.clone(), "getting metadata from", x)
            })
    }

    pub fn try_clone(&self) -> Result<File> {
        self.inner
            .try_clone()
            .map(|x| File::new(x, self.path.clone()))
            .map_err(|x| {
                Error::Filesystem(self.path.clone(), "cloning handle for", x)
            })
    }

    pub fn set_permissions(&self, perm: Permissions) -> Result<()> {
        // TODO: include `perm` in the error?
        self.inner.set_permissions(perm).map_err(|x| {
            Error::Filesystem(self.path.clone(), "setting permissions for", x)
        })
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        self.inner.read(buf).map_err(|x| {
            ::std::io::Error::new(
                x.kind(),
                Error::Filesystem(self.path.clone(), "reading", x),
            )
        })
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> ::std::io::Result<usize> {
        self.inner.read_to_end(buf).map_err(|x| {
            ::std::io::Error::new(
                x.kind(),
                Error::Filesystem(self.path.clone(), "reading", x),
            )
        })
    }

    fn read_to_string(&mut self, buf: &mut String) -> ::std::io::Result<usize> {
        self.inner.read_to_string(buf).map_err(|x| {
            ::std::io::Error::new(
                x.kind(),
                Error::Filesystem(self.path.clone(), "reading", x),
            )
        })
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> ::std::io::Result<()> {
        self.inner.read_exact(buf).map_err(|x| {
            ::std::io::Error::new(
                x.kind(),
                Error::Filesystem(self.path.clone(), "reading", x),
            )
        })
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        self.inner.write(buf).map_err(|x| {
            ::std::io::Error::new(
                x.kind(),
                Error::Filesystem(self.path.clone(), "writing", x),
            )
        })
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        self.inner.flush().map_err(|x| {
            ::std::io::Error::new(
                x.kind(),
                Error::Filesystem(self.path.clone(), "flushing", x),
            )
        })
    }

    fn write_all(&mut self, buf: &[u8]) -> ::std::io::Result<()> {
        self.inner.write_all(buf).map_err(|x| {
            ::std::io::Error::new(
                x.kind(),
                Error::Filesystem(self.path.clone(), "writing", x),
            )
        })
    }

    fn write_fmt(&mut self, fmt: Arguments) -> ::std::io::Result<()> {
        self.inner.write_fmt(fmt).map_err(|x| {
            ::std::io::Error::new(
                x.kind(),
                Error::Filesystem(self.path.clone(), "writing", x),
            )
        })
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> ::std::io::Result<u64> {
        self.inner.seek(pos).map_err(|x| {
            ::std::io::Error::new(
                x.kind(),
                Error::Filesystem(self.path.clone(), "seeking", x),
            )
        })
    }
}

impl Deref for File {
    type Target = ::std::fs::File;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for File {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Wrapper<::std::fs::File> for File {
    fn into_inner(self) -> ::std::fs::File {
        self.inner
    }
}

pub type FileType = ::std::fs::FileType;

pub struct Metadata {
    inner: ::std::fs::Metadata,
    path: PathBuf,
}

impl Metadata {
    fn new(inner: ::std::fs::Metadata, path: PathBuf) -> Self {
        Self { inner, path }
    }

    pub fn modified(&self) -> Result<SystemTime> {
        self.inner.modified().map_err(|x| {
            Error::Filesystem(
                self.path.to_owned(),
                "getting the modification time of",
                x,
            )
        })
    }

    pub fn accessed(&self) -> Result<SystemTime> {
        self.inner.accessed().map_err(|x| {
            Error::Filesystem(
                self.path.to_owned(),
                "getting the access time of",
                x,
            )
        })
    }

    pub fn created(&self) -> Result<SystemTime> {
        self.inner.created().map_err(|x| {
            Error::Filesystem(
                self.path.to_owned(),
                "getting the creation time of",
                x,
            )
        })
    }
}

impl Deref for Metadata {
    type Target = ::std::fs::Metadata;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Wrapper<::std::fs::Metadata> for Metadata {
    fn into_inner(self) -> ::std::fs::Metadata {
        self.inner
    }
}

pub struct OpenOptions(::std::fs::OpenOptions);

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions(::std::fs::OpenOptions::new())
    }

    pub fn read(&mut self, read: bool) -> &mut Self {
        self.0.read(read);

        self
    }

    pub fn write(&mut self, write: bool) -> &mut Self {
        self.0.write(write);

        self
    }

    pub fn append(&mut self, append: bool) -> &mut Self {
        self.0.append(append);

        self
    }

    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.0.truncate(truncate);

        self
    }

    pub fn create(&mut self, create: bool) -> &mut Self {
        self.0.create(create);

        self
    }

    pub fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.0.create_new(create_new);

        self
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<File> {
        let path = path.as_ref();

        // TODO: include the opening options?
        self.0
            .open(path)
            .map(|x| File::new(x, path.to_owned()))
            .map_err(|x| Error::Filesystem(path.to_owned(), "opening file", x))
    }
}

impl Deref for OpenOptions {
    type Target = ::std::fs::OpenOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OpenOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Wrapper<::std::fs::OpenOptions> for OpenOptions {
    fn into_inner(self) -> ::std::fs::OpenOptions {
        self.0
    }
}

pub type Permissions = ::std::fs::Permissions;

pub struct ReadDir {
    inner: ::std::fs::ReadDir,
    path: PathBuf,
}

impl ReadDir {
    fn new(inner: ::std::fs::ReadDir, path: PathBuf) -> Self {
        Self { inner, path }
    }
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| {
            x.map(DirEntry::new).map_err(|err| {
                Error::Filesystem(
                    self.path.to_owned(),
                    "iterating through directory",
                    err,
                )
            })
        })
    }
}

impl Deref for ReadDir {
    type Target = ::std::fs::ReadDir;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ReadDir {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Wrapper<::std::fs::ReadDir> for ReadDir {
    fn into_inner(self) -> ::std::fs::ReadDir {
        self.inner
    }
}

pub fn canonicalize<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();

    ::std::fs::canonicalize(path)
        .map_err(|x| Error::Filesystem(path.to_owned(), "canonicalizing", x))
}

pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64> {
    let from = from.as_ref();
    let to = to.as_ref();

    ::std::fs::copy(from, to).map_err(|x| {
        Error::Filesystem2(from.to_owned(), to.to_owned(), "copying file", x)
    })
}

pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    ::std::fs::create_dir(path).map_err(|x| {
        Error::Filesystem(path.to_owned(), "creating directory", x)
    })
}

pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    ::std::fs::create_dir_all(path).map_err(|x| {
        Error::Filesystem(path.to_owned(), "recursively creating directory", x)
    })
}

pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    ::std::fs::hard_link(src, dst).map_err(|x| {
        Error::Filesystem2(src.to_owned(), dst.to_owned(), "hard linking", x)
    })
}

pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    let path = path.as_ref();

    ::std::fs::metadata(path)
        .map(|x| Metadata::new(x, path.to_owned()))
        .map_err(|x| {
            Error::Filesystem(path.to_owned(), "getting metadata from", x)
        })
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();

    ::std::fs::read(path)
        .map_err(|x| Error::Filesystem(path.to_owned(), "reading", x))
}

pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir> {
    let path = path.as_ref();

    ::std::fs::read_dir(path)
        .map(|x| ReadDir::new(x, path.to_owned()))
        .map_err(|x| Error::Filesystem(path.to_owned(), "reading directory", x))
}

pub fn read_link<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();

    ::std::fs::read_link(path)
        .map_err(|x| Error::Filesystem(path.to_owned(), "reading link", x))
}

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();

    ::std::fs::read_to_string(path)
        .map_err(|x| Error::Filesystem(path.to_owned(), "reading", x))
}

pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    ::std::fs::remove_dir(path).map_err(|x| {
        Error::Filesystem(path.to_owned(), "removing directory", x)
    })
}

pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    ::std::fs::remove_dir_all(path).map_err(|x| {
        Error::Filesystem(path.to_owned(), "recursively removing directory", x)
    })
}

pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    ::std::fs::remove_file(path)
        .map_err(|x| Error::Filesystem(path.to_owned(), "removing file", x))
}

pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    ::std::fs::rename(from, to).map_err(|x| {
        Error::Filesystem2(from.to_owned(), to.to_owned(), "renaming", x)
    })
}

pub fn set_permissions<P: AsRef<Path>>(
    path: P,
    perm: Permissions,
) -> Result<()> {
    let path = path.as_ref();

    // TODO: include `perm` in the error?
    ::std::fs::set_permissions(path, perm).map_err(|x| {
        Error::Filesystem(path.to_owned(), "setting permissions for", x)
    })
}

pub fn soft_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    #[allow(deprecated)]
    ::std::fs::soft_link(src, dst).map_err(|x| {
        Error::Filesystem2(src.to_owned(), dst.to_owned(), "soft linking", x)
    })
}

pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    let path = path.as_ref();

    ::std::fs::metadata(path)
        .map(|x| Metadata::new(x, path.to_owned()))
        .map_err(|x| {
            Error::Filesystem(
                path.to_owned(),
                "getting symlink metadata from",
                x,
            )
        })
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> Result<()> {
    let path = path.as_ref();

    ::std::fs::write(path, contents)
        .map_err(|x| Error::Filesystem(path.to_owned(), "writing", x))
}
