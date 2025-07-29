use std::io::{Read, Seek};
use std::path::Path;

use zip::ZipArchive;
use zip::read::ZipFile;
use zip::result::{ZipError, ZipResult};

pub trait ByPath<R>
where
    R: Read,
{
    /// Returns a file from the ZIP archive by its path.
    fn by_path<T>(&mut self, path: T) -> ZipResult<ZipFile<'_, R>>
    where
        T: AsRef<Path>;
}

impl<R> ByPath<R> for ZipArchive<R>
where
    R: Read + Seek,
{
    fn by_path<T>(&mut self, path: T) -> ZipResult<ZipFile<'_, R>>
    where
        T: AsRef<Path>,
    {
        self.index_for_path(path)
            .ok_or(ZipError::FileNotFound)
            .and_then(|index| self.by_index(index))
    }
}
