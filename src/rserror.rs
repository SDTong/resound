//! error

use std::{error::Error, fmt, result};

pub(crate) type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub(crate) type Result<T> = result::Result<T, GenericError>;

pub struct RsError {
    pub msg: String,
    source: Option<GenericError>,
}

impl RsError {
    /// 生成AudioError
    /// msg 是一般的错误信息，
    pub(crate) fn with_msg<T>(msg: T) -> RsError
    where
        T: Into<String>,
    {
        RsError {
            msg: msg.into(),
            source: None,
        }
    }
}

impl fmt::Debug for RsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RsError").field("msg", &self.msg).finish()
    }
}

impl fmt::Display for RsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for RsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(err) => Some(err.as_ref()),
            None => None,
        }
    }
}
