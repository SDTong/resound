//! error

use std::{error::Error, fmt, result};

pub(crate) type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub(crate) type Result<T> = result::Result<T, AudioError>;

pub struct AudioError {
    pub msg: String,
    source: Option<GenericError>,
}

impl AudioError {
    // 生成AudioError
    // msg 是一般的错误信息，
    pub(crate) fn with_msg<T>(msg: T) -> AudioError
    where
        T: Into<String>,
    {
        AudioError {
            msg: msg.into(),
            source: None,
        }
    }
    // 生成AudioError
    // msg 是一般的错误信息，
    // status_msg 是 status 翻译出来的错误信息
    // status 是操作系统 core audio 等框架函数返回的错误码
    pub(crate) fn with_status_msg<T>(
        msg: T,
        status_msg: T,
        status: coreaudio_sys::OSStatus,
    ) -> AudioError
    where
        T: Into<String>,
    {
        AudioError {
            msg: format!(
                "{}: {}[OSStatus: {}]",
                msg.into(),
                status_msg.into(),
                status
            ),
            source: None,
        }
    }
}

impl fmt::Debug for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioError")
            .field("msg", &self.msg)
            .finish()
    }
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for AudioError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(err) => Some(err.as_ref()),
            None => None,
        }
    }
}
