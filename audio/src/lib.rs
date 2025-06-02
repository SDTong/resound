//! provide macos audio

pub mod aoerror;
mod core_audio;
mod foundation;

use aoerror::{AudioError, Result};
use std::cell;

pub use core_audio::aggregate_device;
pub use core_audio::device;
pub use core_audio::ext_audio_file;
pub use core_audio::process;
pub use core_audio::stream;
pub use core_audio::tap;

pub use core_audio::K_AUDIO_HARDWARE_NO_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_NOT_RUNNING_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_UNSPECIFIED_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_UNKNOWN_PROPERTY_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_BAD_PROPERTY_SIZE_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_ILLEGAL_OPERATION_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_BAD_OBJECT_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_BAD_DEVICE_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_BAD_STREAM_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_UNSUPPORTED_OPERATION_ERROR;
pub use core_audio::K_AUDIO_HARDWARE_NOT_READY_ERROR;
pub use core_audio::K_AUDIO_DEVICE_UNSUPPORTED_FORMAT_ERROR;
pub use core_audio::K_AUDIO_DEVICE_PERMISSIONS_ERROR;

// 统一外部模块使用的定义
pub type AudioObjectId = coreaudio_sys::AudioObjectID;
pub type AudioStreamBasicDescription = coreaudio_sys::AudioStreamBasicDescription;
pub type AudioTimeStamp = coreaudio_sys::AudioTimeStamp;
pub type AudioBufferList = coreaudio_sys::AudioBufferList;
pub type OSStatus = coreaudio_sys::OSStatus;

// 标记 c_void 和 core foundtion 中的 xxRef
pub(crate) trait Ref {}
impl Ref for *const std::ffi::c_void {}
impl Ref for coreaudio_sys::CFNumberRef {}
impl Ref for coreaudio_sys::CFStringRef {}
impl Ref for coreaudio_sys::CFDictionaryRef {}

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
