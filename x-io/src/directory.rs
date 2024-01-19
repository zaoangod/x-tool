use std::fs::{create_dir_all, remove_dir_all};

use crate::error::FsIOError;
use crate::path::as_path::AsPath;
use crate::path::parent_directory;
use crate::result::FsIOResult;

/// 创建一个目录
///
/// # 参数
///
/// * `path` - 目录路径
///
/// # 示例
///
/// use crate::x-io::directory;
/// use std::path::Path;
/// fn main() {
///     let result = directory::create("./target/__test/directory_test/dir1/dir2");
///     assert!(result.is_ok());
///
///     let path = Path::new("./target/__test/directory_test/dir1/dir2");
///     assert!(path.exists());
/// }
pub fn create<T: AsPath + ?Sized>(path: &T) -> FsIOResult<()> {
    let directory_path = path.as_path();

    if directory_path.is_dir() && directory_path.exists() {
        return Ok(());
    }

    match create_dir_all(&directory_path) {
        Ok(_) => Ok(()),
        Err(error) => Err(FsIOError::IOError(
            format!("Unable to create directory: {:?}.", &directory_path).to_string(),
            Some(error),
        )),
    }
}

/// 给提供的路径创建父级目录
///
/// # 参数
///
/// * `path` - 目录路径
///
/// # 示例
///
/// ```
/// use crate::x_io::directory;
/// use std::path::Path;
///
/// fn main() {
///     let result = directory::create_parent("./target/__test/directory_test/dir1/files/file.txt");
///     assert!(result.is_ok());
///
///     let path = Path::new("./target/__test/directory_test/dir1/files");
///     assert!(path.exists());
/// }
/// ```
pub fn create_parent<T: AsPath + ?Sized>(path: &T) -> FsIOResult<()> {
    match parent_directory(path) {
        Some(directory) => create(&directory),
        None => Ok(()),
    }
}

/// 删除该目录和任何子文件目录
///
/// # 参数
///
/// * `path` - 路径
///
/// # 示例
///
/// ```
/// use crate::x_io::directory;
/// use std::path::Path;
/// use x_io::file;
///
/// fn main() {
///     file::ensure_exists("./target/__test/directory_test/delete_directory/dir1/dir2/file.txt").unwrap();
///     let path = Path::new("./target/__test/directory_test/delete_directory");
///     assert!(path.exists());
///
///     let result = directory::delete("./target/__test/directory_test/delete_directory");
///     assert!(result.is_ok());
///
///     assert!(!path.exists());
/// }
/// ```
pub fn delete<T: AsPath + ?Sized>(path: &T) -> FsIOResult<()> {
    let directory_path = path.as_path();

    if directory_path.exists() {
        if directory_path.is_dir() {
            match remove_dir_all(directory_path) {
                Ok(_) => Ok(()),
                Err(error) => Err(FsIOError::IOError(
                    format!("Unable to delete directory: {:?}", &directory_path).to_string(),
                    Some(error),
                )),
            }
        } else {
            Err(FsIOError::NotFile(
                format!("Path: {:?} is not a directory.", &directory_path).to_string(),
            ))
        }
    } else {
        Ok(())
    }
}