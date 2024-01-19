use std::fs::{File, OpenOptions, read, read_to_string, remove_file};
use std::io;
use std::io::Write;

use crate::directory;
use crate::error::FsIOError;
use crate::path::as_path::AsPath;
use crate::result::FsIOResult;

/// 保证文件存在(文件不存在会创建一个空的文件)
///
/// # 参数
///
/// * `path` - 文件路径
///
/// # 示例
///
/// ```
/// use crate::x_io::file;
/// use std::path::Path;
///
/// fn main() {
///     let result = file::ensure_exists("./target/__test/file_test/dir1/dir2/file.txt");
///     assert!(result.is_ok());
///
///     let path = Path::new("./target/__test/file_test/dir1/dir2/file.txt");
///     assert!(path.exists());
/// }
/// ```
pub fn ensure_exists<T: AsPath + ?Sized>(path: &T) -> FsIOResult<()> {
    let file_path = path.as_path();

    if file_path.exists() {
        if file_path.is_file() {
            Ok(())
        } else {
            Err(FsIOError::AlreadyExist(
                format!("Unable to create file: {:?}", &file_path).to_string(),
            ))
        }
    } else {
        directory::create_parent(path)?;

        match File::create(&file_path) {
            Ok(_) => Ok(()),
            Err(error) => Err(FsIOError::IOError(
                format!("Unable to create file: {:?}", &file_path).to_string(),
                Some(error),
            )),
        }
    }
}

/// 创建文本文件, 如果存在则覆盖文件
///
/// # 参数
///
/// * `path` - 文件路径
/// * `text` - 文本内容
///
/// # 示例
///
/// ```
/// use crate::x_io::file;
/// use std::path::Path;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/write_text_file/file.txt";
///     let result = file::write_text_file(file_path, "some content");
///     assert!(result.is_ok());
///
///     let text = file::read_text_file(file_path).unwrap();
///
///     assert_eq!(text, "some content");
/// }
/// ```
pub fn write_text_file<T: AsPath + ?Sized>(path: &T, text: &str) -> FsIOResult<()> {
    write_file(path, text.as_bytes())
}

/// 追加文本文件, 如果存在则追加内容
///
/// # 参数
///
/// * `path` - 文件路径
/// * `text` - 文本内容
///
/// # 示例
///
/// ```
/// use crate::x_io::file;
///
///
/// fn main() {
///     let file_path = "./target/__test/file_test/append_text_file/file.txt";
///     let mut result = file::write_text_file(file_path, "some content");
///     assert!(result.is_ok());
///     result = file::append_text_file(file_path, "more content");
///     assert!(result.is_ok());
///
///     let text = file::read_text_file(file_path).unwrap();
///
///     assert_eq!(text, "some content more content");
/// }
/// ```
pub fn append_text_file<T: AsPath + ?Sized>(path: &T, text: &str) -> FsIOResult<()> {
    append_file(path, text.as_bytes())
}

/// 创建原始数据, 如果存在则覆盖文件
///
/// # 参数
///
/// * `path` - 文件路径
/// * `text` - 文本内容
///
/// # 示例
///
/// ```
/// use crate::x_io::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/write_file/file.txt";
///     let mut result = file::write_file(file_path, "some content".as_bytes());
///     assert!(result.is_ok());
///     result = file::append_file(file_path, "\nmore content".as_bytes());
///     assert!(result.is_ok());
///
///     let data = file::read_file(file_path).unwrap();
///
///     assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
/// }
/// ```
pub fn write_file<T: AsPath + ?Sized>(path: &T, data: &[u8]) -> FsIOResult<()> {
    modify_file(path, &move |file: &mut File| file.write_all(data), false)
}

/// 追加原始数据, 如果存在则追加文件
///
/// # 参数
///
/// * `path` - 文件路径
/// * `text` - 文本内容
///
/// # 示例
///
/// ```
/// use crate::x_io::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/append_file/file.txt";
///     let mut result = file::write_file(file_path, "some content".as_bytes());
///     assert!(result.is_ok());
///     result = file::append_file(file_path, "\nmore content".as_bytes());
///     assert!(result.is_ok());
///
///     let data = file::read_file(file_path).unwrap();
///
///     assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
/// }
/// ```
pub fn append_file<T: AsPath + ?Sized>(path: &T, data: &[u8]) -> FsIOResult<()> {
    modify_file(path, &move |file: &mut File| file.write_all(data), true)
}

/// 覆盖或追加文件, 并触发提供的 write_content 函数以启用自定义写入
///
/// # 参数
///
/// * `path` - 文件路径
/// * `write_content` - 自定义写入方法
/// * `append` - True表示追加, false表示覆盖
///
/// # 示例
///
/// ```
/// use crate::x_io::file;
/// use std::fs::File;
/// use std::io::Write;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/modify_file/file.txt";
///     let mut result = file::modify_file(
///         file_path,
///         &move |file: &mut File| file.write_all("some content".as_bytes()),
///         false,
///     );
///     assert!(result.is_ok());
///     result = file::modify_file(
///         file_path,
///         &move |file: &mut File| file.write_all("\nmore content".as_bytes()),
///         true,
///     );
///     assert!(result.is_ok());
///
///     let data = file::read_file(file_path).unwrap();
///
///     assert_eq!(str::from_utf8(&data).unwrap(), "some content\nmore content");
/// }
/// ```
pub fn modify_file<T: AsPath + ?Sized>(
    path: &T,
    write_content: &dyn Fn(&mut File) -> io::Result<()>,
    append: bool,
) -> FsIOResult<()> {
    directory::create_parent(path)?;

    let file_path = path.as_path();

    // create or open
    let result = if append && file_path.exists() {
        OpenOptions::new().append(true).open(file_path)
    } else {
        File::create(&file_path)
    };

    match result {
        Ok(mut fd) => match write_content(&mut fd) {
            Ok(_) => match fd.sync_all() {
                Ok(_) => Ok(()),
                Err(error) => Err(FsIOError::IOError(
                    format!("Error finish up writing to file: {:?}", &file_path).to_string(),
                    Some(error),
                )),
            },
            Err(error) => Err(FsIOError::IOError(
                format!("Error while writing to file: {:?}", &file_path).to_string(),
                Some(error),
            )),
        },
        Err(error) => Err(FsIOError::IOError(
            format!("Unable to create/open file: {:?} for writing.", &file_path).to_string(),
            Some(error),
        )),
    }
}

/// 读取文本文件内容
///
/// # 参数
///
/// * `path` - 文件路径
///
/// # 示例
///
/// ```
/// use crate::x_io::file;
/// use std::path::Path;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/write_text_file/file.txt";
///     let result = file::write_text_file(file_path, "some content");
///     assert!(result.is_ok());
///
///     let text = file::read_text_file(file_path).unwrap();
///
///     assert_eq!(text, "some content");
/// }
/// ```
pub fn read_text_file<T: AsPath + ?Sized>(path: &T) -> FsIOResult<String> {
    let file_path = path.as_path();

    match read_to_string(&file_path) {
        Ok(content) => Ok(content),
        Err(error) => Err(FsIOError::IOError(
            format!("Unable to read file: {:?}", &file_path).to_string(),
            Some(error),
        )),
    }
}

/// 读取文件内容
///
/// # Arguments
///
/// * `path` - The file path
///
/// # Example
///
/// ```
/// use crate::x_io::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/read_file/file.txt";
///     let mut result = file::write_file(file_path, "some content".as_bytes());
///     assert!(result.is_ok());
///     result = file::append_file(file_path, "more content".as_bytes());
///     assert!(result.is_ok());
///
///     let data = file::read_file(file_path).unwrap();
///
///     assert_eq!(str::from_utf8(&data).unwrap(), "some content more content");
/// }
/// ```
pub fn read_file<T: AsPath + ?Sized>(path: &T) -> FsIOResult<Vec<u8>> {
    let file_path = path.as_path();

    match read(&file_path) {
        Ok(content) => Ok(content),
        Err(error) => Err(FsIOError::IOError(
            format!("Unable to read file: {:?}", &file_path).to_string(),
            Some(error),
        )),
    }
}

/// 删除文件, 如果文件不存在返回Ok
///
/// # 参数
///
/// * `path` - 文件路径
///
/// # 示例
///
/// ```
/// use crate::x_io::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/delete_file/file.txt";
///     let mut result = file::ensure_exists(file_path);
///     assert!(result.is_ok());
///
///     let path = Path::new(file_path);
///     assert!(path.exists());
///
///     result = file::delete(file_path);
///     assert!(result.is_ok());
///
///     assert!(!path.exists());
/// }
/// ```
pub fn delete<T: AsPath + ?Sized>(path: &T) -> FsIOResult<()> {
    let file_path = path.as_path();

    if file_path.exists() {
        if file_path.is_file() {
            match remove_file(file_path) {
                Ok(_) => Ok(()),
                Err(error) => Err(FsIOError::IOError(
                    format!("Unable to delete file: {:?}", &file_path).to_string(),
                    Some(error),
                )),
            }
        } else {
            Err(FsIOError::NotFile(
                format!("Path: {:?} is not a file.", &file_path).to_string(),
            ))
        }
    } else {
        Ok(())
    }
}

/// 删除文件, 如果文件不存在返回true
///
/// # 参数
///
/// * `path` - 文件路径
///
/// # 示例
///
/// ```
/// use crate::x_io::file;
/// use std::path::Path;
/// use std::str;
///
/// fn main() {
///     let file_path = "./target/__test/file_test/delete_file/file.txt";
///     let result = file::ensure_exists(file_path);
///     assert!(result.is_ok());
///
///     let path = Path::new(file_path);
///     assert!(path.exists());
///
///     let deleted = file::delete_ignore_error(file_path);
///     assert!(deleted);
///
///     assert!(!path.exists());
/// }
/// ```
pub fn delete_ignore_error<T: AsPath + ?Sized>(path: &T) -> bool {
    match delete(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}