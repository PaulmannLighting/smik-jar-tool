use std::io::{Read, Seek};
use std::path::PathBuf;
use std::vec::IntoIter;

use zip::ZipArchive;
use zip::read::ZipFile;

use crate::JarError;

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
    pub fn next(&mut self) -> Option<(PathBuf, Result<ZipFile<'_, T>, JarError>)> {
        let path = self.file_names.next()?;

        let Some(file_name) = path.to_str() else {
            return Some((path.clone(), Err(JarError::Utf8(path))));
        };

        let zip_result = self.zip_archive.by_name(file_name);
        Some((
            path,
            match zip_result {
                Ok(zip_file) => Ok(zip_file),
                Err(error) => Err(JarError::Zip(error)),
            },
        ))
    }
}
