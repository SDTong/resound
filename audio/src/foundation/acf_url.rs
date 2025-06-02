//! provide CFURL operate

use std::{os::unix::ffi::OsStrExt as _, path::Path, ptr};

use coreaudio_sys::{CFURLCreateFromFileSystemRepresentation, CFURLRef};

use super::acf_index::CFIndexConvertible as _;

use crate::aoerror::{AudioError, Result};

// 和macos的core audio 框架交互时，部分函数会获取 CFURLRef 的所有权，
// 所以只提供crate内部函数，不做rust风格封装
// 会拷贝字节，如果不把所有权交给其它函数，那么调用者需要释放内存
// 文件操作失败风险较高，返回Result，强制调用者校验
pub(crate) fn create_cf_url_ref<P: AsRef<Path>>(path: P, is_directory: bool) -> Result<CFURLRef> {
    let path_bytes;
    #[cfg(unix)]
    {
        path_bytes = path.as_ref().as_os_str().as_bytes()
    }
    #[cfg(not(unix))]
    {
        // XXX: Getting non-valid UTF8 paths into CoreFoundation on Windows is going to be unpleasant
        // CFURLGetWideFileSystemRepresentation might help
        path_bytes = match path.as_ref().to_str() {
            Some(path) => path,
            None => return None,
        }
    }

    unsafe {
        let url_ref = CFURLCreateFromFileSystemRepresentation(
            ptr::null_mut(),
            path_bytes.as_ptr(),
            path_bytes.len().to_cfindex(),
            is_directory as u8,
        );
        if url_ref.is_null() {
            Err(AudioError::with_msg("create CFURLRef fail"))
        } else {
            Ok(url_ref)
        }
    }
}
