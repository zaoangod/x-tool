//! # error
//!
//! The error structure and types.
//!

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::time::SystemTimeError;

/// 保存错误信息
#[derive(Debug)]
pub enum FsIOError {
    /// 路径已存在
    AlreadyExist(String),
    /// 不是文件
    NotFile(String),
    /// IO错误
    IOError(String, Option<io::Error>),
    /// 系统时间错误
    SystemTimeError(String, Option<SystemTimeError>),
}

impl Display for FsIOError {
    /// 格式化错误
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::AlreadyExist(ref message) => write!(formatter, "{}", message),
            Self::NotFile(ref message) => write!(formatter, "{}", message),
            Self::IOError(ref message, ref cause) => {
                writeln!(formatter, "{}", message)?;
                match cause {
                    Some(cause_err) => cause_err.fmt(formatter),
                    None => Ok(()),
                }
            }
            Self::SystemTimeError(ref message, ref cause) => {
                writeln!(formatter, "{}", message)?;
                match cause {
                    Some(cause_err) => cause_err.fmt(formatter),
                    None => Ok(()),
                }
            }
        }
    }
}

impl Error for FsIOError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::AlreadyExist(_) => None,
            Self::NotFile(_) => None,
            Self::IOError(_, error) => error.as_ref().map(|io_error| {
                let std_error: &dyn Error = io_error;
                std_error
            }),
            Self::SystemTimeError(_, error) => error.as_ref().map(|system_time_error| {
                let std_error: &dyn Error = system_time_error;
                std_error
            }),
        }
    }
}