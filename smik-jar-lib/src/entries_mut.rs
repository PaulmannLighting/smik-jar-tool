use std::io::{Read, Seek};
use std::path::PathBuf;
use std::vec::IntoIter;

use zip::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipResult;

/// A _lending iterator_ over the entries in a ZIP archive.
pub struct EntriesMut<'a, T> {
    zip_archive: &'a mut ZipArchive<T>,
    file_names: IntoIter<PathBuf>,
}

impl<'a, T> EntriesMut<'a, T> {
    pub fn new(zip_archive: &'a mut ZipArchive<T>, file_names: Vec<PathBuf>) -> Self {
        Self {
            zip_archive,
            file_names: file_names.into_iter(),
        }
    }
}

impl<T> EntriesMut<'_, T>
where
    T: Read + Seek,
{
    /// Return the next entry in the ZIP archive.
    ///
    /// Since [`ZipFile`] needs a lifetime related to the borrow of `self`, this constitutes a
    /// _lending iterator_ and thus cannot be implemented as an [`Iterator`].
    pub fn next(&mut self) -> Option<(PathBuf, ZipResult<ZipFile<'_, T>>)> {
        let path = self.file_names.next()?;
        let file_name = path.as_os_str().to_str()?;
        let zip_result = self.zip_archive.by_name(file_name);
        Some((path, zip_result))
    }
}
