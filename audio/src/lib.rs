//! provide macos audio

mod core_audio;
mod error;
mod foundation;

use error::{AudioError, Result};
use std::cell;

pub use core_audio::process;
pub use core_audio::tap;
// 统一外部模块使用的定义
pub type AudioObjectId = coreaudio_sys::AudioObjectID;

fn get_or_try_init<'a, T, F>(once_cell: &'a cell::OnceCell<T>, f: F) -> Result<&'a T>
where
    F: FnOnce() -> Result<T>,
{
    if let Some(val) = once_cell.get() {
        return Ok(val);
    }
    let val = f()?;
    if let Err(_) = once_cell.set(val) {
        // 已经初始化了
        // OnceCell不是线程安全的，这个结果说明调用者错误使用，
        // 因为涉及创建Tap等macos的core audio相关东西，如果panic，会导致没有删除，选择返回Err
        return Err(AudioError::with_msg("OnceCell init multiple times"));
    }
    once_cell
        .get()
        .ok_or(AudioError::with_msg("OnceCell init fail"))
}
