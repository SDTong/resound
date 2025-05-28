//! core audio Framework 支持

use std::{ffi::c_void, mem, ptr::null};

use coreaudio_sys::{AudioObjectID, AudioObjectPropertyAddress, CFStringRef, UInt32};

use crate::{Result, foundation};

// 检查 AudioHardwareBase.h 函数结果
macro_rules! check_status {
    ($msg:expr, $status:expr) => {
        if $status != crate::core_audio::K_AUDIO_HARDWARE_NO_ERROR {
            return Err(crate::aoerror::AudioError::with_status_msg(
                $msg,
                crate::core_audio::err_msg_hardware_status($status),
                $status,
            ));
        }
    };
}

// 检查core audio函数执行结果，不抛出异常
// 用于自定义drop函数中，检查删除结果
macro_rules! eprintln_status {
    ($msg:expr, $status:expr) => {
        if $status != crate::core_audio::K_AUDIO_HARDWARE_NO_ERROR {
            let status_msg = crate::core_audio::err_msg_hardware_status($status);
            eprintln!("{}: {}[OSStatus: {}]", $msg, status_msg, $status);
        }
    };
}

pub mod aggregate_device;
pub mod process;
pub mod tap;

const CF_STR_REF_SIZE: UInt32 = mem::size_of::<CFStringRef>() as UInt32;

// AudioHardwareBase.h 头文件中定义的错误 start
// coreaudio_sys中，使用u32定义错误值，导致不能使用match，
// 从代码看，头文件中使用OSStatus定义，是i32，
// 所以，重新定义错误码
pub const K_AUDIO_HARDWARE_NO_ERROR: coreaudio_sys::OSStatus = 0;
pub const K_AUDIO_HARDWARE_NOT_RUNNING_ERROR: coreaudio_sys::OSStatus = 1937010544;
pub const K_AUDIO_HARDWARE_UNSPECIFIED_ERROR: coreaudio_sys::OSStatus = 2003329396;
pub const K_AUDIO_HARDWARE_UNKNOWN_PROPERTY_ERROR: coreaudio_sys::OSStatus = 2003332927;
pub const K_AUDIO_HARDWARE_BAD_PROPERTY_SIZE_ERROR: coreaudio_sys::OSStatus = 561211770;
pub const K_AUDIO_HARDWARE_ILLEGAL_OPERATION_ERROR: coreaudio_sys::OSStatus = 1852797029;
pub const K_AUDIO_HARDWARE_BAD_OBJECT_ERROR: coreaudio_sys::OSStatus = 560947818;
pub const K_AUDIO_HARDWARE_BAD_DEVICE_ERROR: coreaudio_sys::OSStatus = 560227702;
pub const K_AUDIO_HARDWARE_BAD_STREAM_ERROR: coreaudio_sys::OSStatus = 561214578;
pub const K_AUDIO_HARDWARE_UNSUPPORTED_OPERATION_ERROR: coreaudio_sys::OSStatus = 1970171760;
pub const K_AUDIO_HARDWARE_NOT_READY_ERROR: coreaudio_sys::OSStatus = 1852990585;
pub const K_AUDIO_DEVICE_UNSUPPORTED_FORMAT_ERROR: coreaudio_sys::OSStatus = 560226676;
pub const K_AUDIO_DEVICE_PERMISSIONS_ERROR: coreaudio_sys::OSStatus = 560492391;
// AudioHardwareBase.h 头文件中定义的错误 end

// 翻译错误码, AudioHardwareBase.h 头文件中的
fn err_msg_hardware_status(status: coreaudio_sys::OSStatus) -> &'static str {
    match status {
        K_AUDIO_HARDWARE_NO_ERROR => {
            "The function call completed successfully.[kAudioHardwareNoError: 0]"
        }
        K_AUDIO_HARDWARE_NOT_RUNNING_ERROR => {
            "The function call requires that the hardware be running but it isn't.[kAudioHardwareNotRunningError: stop]"
        }
        K_AUDIO_HARDWARE_UNSPECIFIED_ERROR => {
            "The function call failed while doing something that doesn't provide any error messages.[kAudioHardwareUnspecifiedError: what]"
        }
        K_AUDIO_HARDWARE_UNKNOWN_PROPERTY_ERROR => {
            "The AudioObject doesn't know about the property at the given address.[kAudioHardwareUnknownPropertyError: who?]"
        }
        K_AUDIO_HARDWARE_BAD_PROPERTY_SIZE_ERROR => {
            "An improperly sized buffer was provided when accessing the data of a property.[kAudioHardwareBadPropertySizeError: !siz]"
        }
        K_AUDIO_HARDWARE_ILLEGAL_OPERATION_ERROR => {
            "The requested operation couldn't be completed.[kAudioHardwareIllegalOperationError: nope]"
        }
        K_AUDIO_HARDWARE_BAD_OBJECT_ERROR => {
            "The AudioObjectID passed to the function doesn't map to a valid AudioObject.[kAudioHardwareBadObjectError: !obj]"
        }
        K_AUDIO_HARDWARE_BAD_DEVICE_ERROR => {
            "The AudioObjectID passed to the function doesn't map to a valid AudioDevice.[kAudioHardwareBadDeviceError: !dev]"
        }
        K_AUDIO_HARDWARE_BAD_STREAM_ERROR => {
            "The AudioObjectID passed to the function doesn't map to a valid AudioStream.[kAudioHardwareBadStreamError: !str]"
        }
        K_AUDIO_HARDWARE_UNSUPPORTED_OPERATION_ERROR => {
            "The AudioObject isn't ready to do the requested operation.[kAudioHardwareUnsupportedOperationError: unop]"
        }
        K_AUDIO_HARDWARE_NOT_READY_ERROR => {
            "The AudioObject isn't ready to do the requested operation[kAudioHardwareNotReadyError: nrdy]"
        }
        K_AUDIO_DEVICE_UNSUPPORTED_FORMAT_ERROR => {
            "The AudioStream doesn't support the requested format.[kAudioDeviceUnsupportedFormatError: !dat]"
        }
        K_AUDIO_DEVICE_PERMISSIONS_ERROR => {
            "The requested operation can't be completed because the process doesn't have permission.[kAudioDevicePermissionsError: !hog]"
        }
        _ => "unknow error[unknow: null]",
    }
}

fn build_property_address(
    selector: coreaudio_sys::AudioObjectPropertySelector,
) -> AudioObjectPropertyAddress {
    build_property_address_all(
        selector,
        coreaudio_sys::kAudioObjectPropertyScopeGlobal,
        coreaudio_sys::kAudioObjectPropertyElementMain,
    )
}

fn build_property_address_all(
    selector: coreaudio_sys::AudioObjectPropertySelector,
    scope: coreaudio_sys::AudioObjectPropertyScope,
    element: coreaudio_sys::AudioObjectPropertyScope,
) -> AudioObjectPropertyAddress {
    let addr = AudioObjectPropertyAddress {
        mSelector: selector,
        mScope: scope,
        mElement: element,
    };
    addr
}

// 单位： byte
fn get_property_data_size(
    object_id: AudioObjectID,
    addr: &AudioObjectPropertyAddress,
) -> Result<UInt32> {
    let mut size = 0;
    let status = unsafe {
        coreaudio_sys::AudioObjectGetPropertyDataSize(object_id, addr, 0, null(), &mut size)
    };
    check_status!("get property size fail", status);
    Ok(size)
}

// get list type property
fn get_property_data_list<T>(
    object_id: AudioObjectID,
    addr: &AudioObjectPropertyAddress,
) -> Result<Vec<T>> {
    let mut size = get_property_data_size(object_id, addr)?;
    // 计算Vec长度
    let len = size as usize / mem::size_of::<T>();
    let mut data_list: Vec<T> = Vec::with_capacity(len);
    let status = unsafe {
        coreaudio_sys::AudioObjectGetPropertyData(
            object_id,
            addr,
            0,
            std::ptr::null(),
            &mut size,
            data_list.as_mut_ptr() as *mut c_void,
        )
    };
    check_status!("get list property data fail", status);
    let len = size as usize / mem::size_of::<T>();
    unsafe { data_list.set_len(len) };

    Ok(data_list)
}

// get string type property
fn get_property_data_string(
    object_id: AudioObjectID,
    addr: &AudioObjectPropertyAddress,
) -> Result<String> {
    let mut cf_str_ref = mem::MaybeUninit::<CFStringRef>::uninit();
    let mut size = CF_STR_REF_SIZE;
    let status = unsafe {
        coreaudio_sys::AudioObjectGetPropertyData(
            object_id,
            addr,
            0,
            std::ptr::null(),
            &mut size,
            cf_str_ref.as_mut_ptr() as *mut std::ffi::c_void,
        )
    };

    check_status!("get string property data fail", status);
    let cf_str_ref = unsafe { cf_str_ref.assume_init() };
    Ok(foundation::into_string_drop(cf_str_ref))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_property_data_size() {
        let addr = build_property_address(coreaudio_sys::kAudioHardwarePropertyProcessObjectList);
        let size = get_property_data_size(coreaudio_sys::kAudioObjectSystemObject, &addr);
        assert!(size.is_ok(), "size: {:?}", size);
    }

    #[test]
    fn test_get_property_data_list() {
        let addr = build_property_address(coreaudio_sys::kAudioHardwarePropertyProcessObjectList);
        let list =
            get_property_data_list::<AudioObjectID>(coreaudio_sys::kAudioObjectSystemObject, &addr);
        assert!(list.is_ok(), "list: {:?}", list);
    }

    #[test]
    fn test_get_property_data_string() {
        let addr = build_property_address(coreaudio_sys::kAudioHardwarePropertyProcessObjectList);
        let list =
            get_property_data_list::<AudioObjectID>(coreaudio_sys::kAudioObjectSystemObject, &addr);
        assert!(list.is_ok(), "list: {:?}", list);
        let list = list.unwrap();
        println!("list:{:?}", list);
        let id = list[0];
        let addr = build_property_address(coreaudio_sys::kAudioProcessPropertyBundleID);
        let s = get_property_data_string(id, &addr);
        assert!(s.is_ok(), "s: {:?}", s);
    }
}
