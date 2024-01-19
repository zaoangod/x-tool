//! # path
//!
//! Path utility functions and traits.
//!
use std::fs;
use std::time::SystemTime;

use as_path::AsPath;
use from_path::FromPath;

use crate::dunce;
use crate::error::FsIOError;
use crate::result::FsIOResult;

pub mod as_path;
pub mod from_path;

/// 规范提供的路径并且返回字符串路径
///
/// # 参数
///
/// * `path` - 路径值
///
/// # 示例
///
/// ```
/// use x_io::path;
/// use x_io::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let path_obj = Path::new("./src/path/mod.rs");
///     let path1 = path::normalize_as_string(&path_obj);
///     let path2 = path::normalize_as_string(&"./src/path/mod.rs".to_string());
///     assert_eq!(path1.unwrap(), path2.unwrap());
/// }
/// ```
pub fn normalize_as_string<T: AsPath + ?Sized>(path: &T) -> FsIOResult<String> {
    let path_obj = path.as_path();

    match path_obj.canonicalize() {
        Ok(path_buf) => {
            let path_string: String = FromPath::from_path(&path_buf);
            #[cfg(not(windows))]
            {
                Ok(path_string)
            }
            #[cfg(windows)]
            {
                let win_path_string = match dunce::canonicalize(&path_string) {
                    Ok(value) => FromPath::from_path(&value),
                    Err(_) => path_string,
                };

                Ok(win_path_string)
            }
        }
        Err(error) => Err(FsIOError::IOError(
            "Unable to canonicalize path.".to_string(),
            Some(error),
        )),
    }
}

/// 规范提供的路径并且返回字符串路径
///
/// # 参数
///
/// * `path` - 路径
///
/// # 示例
///
/// ```
/// use x_io::path;
/// use x_io::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let path_obj = Path::new("./src/path/mod.rs");
///     let path1 = path::normalize_as_string(&path_obj);
///     let path2 = path::canonicalize_or("./src/path/mod.rs", "/src/path/mod.rs");
///     assert_eq!(path1.unwrap(), path2);
/// }
/// ```
pub fn canonicalize_or<T: AsPath + ?Sized>(path: &T, or_value: &str) -> String {
    match normalize_as_string(path) {
        Ok(value) => value,
        Err(_) => or_value.to_string(),
    }
}

/// 返回最后一个路径组件(文件名或最后一个目录名)
///
/// # 参数
///
/// * `path` - 路径
///
/// # 示例
///
/// ```
/// use x_io::path;
/// use x_io::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let basename = path::base_name("./src/path/mod.rs");
///     assert_eq!(basename.unwrap(), "mod.rs");
/// }
/// ```
pub fn base_name<T: AsPath + ?Sized>(path: &T) -> Option<String> {
    let path_obj = path.as_path();
    match path_obj.file_name() {
        Some(name) => Some(name.to_string_lossy().into_owned()),
        None => None,
    }
}

/// 返回父级路径
///
/// # 参数
///
/// * `path` - 路径
///
/// # 示例
///
/// ```
/// use x_io::path;
/// use x_io::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let dirname = path::parent_directory("./src/path/mod.rs");
///     assert_eq!(dirname.unwrap(), "./src/path");
/// }
/// ```
pub fn parent_directory<T: AsPath + ?Sized>(path: &T) -> Option<String> {
    let path_obj = path.as_path();
    let directory = path_obj.parent();
    match directory {
        Some(directory_path) => {
            let directory_path_string: String = FromPath::from_path(directory_path);
            if directory_path_string.is_empty() {
                None
            } else {
                Some(directory_path_string)
            }
        }
        None => None,
    }
}

/// 返回最后修改时间(单位为毫秒)
///
/// # 参数
///
/// * `path` - 路径
///
/// # 示例
///
/// ```
/// use x_io::path;
///
/// fn main() {
///     let time = path::get_last_modified_time("./src/path/mod.rs").unwrap();
///     assert!(time > 0);
/// }
/// ```
pub fn get_last_modified_time(path: &str) -> FsIOResult<u128> {
    match fs::metadata(path) {
        Ok(metadata) => match metadata.modified() {
            Ok(time) => match time.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => Ok(duration.as_millis()),
                Err(error) => Err(FsIOError::SystemTimeError(
                    "Unable to get last modified duration for path.".to_string(),
                    Some(error),
                )),
            },
            Err(error) => Err(FsIOError::IOError(
                "Unable to extract modified time for path.".to_string(),
                Some(error),
            )),
        },
        Err(error) => Err(FsIOError::IOError(
            "Unable to extract metadata for path.".to_string(),
            Some(error)),
        ),
    }
}