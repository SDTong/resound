//! provide CFString operate

use std::{borrow::Cow, ffi::CStr, ptr, str};

use coreaudio_sys::{
    Boolean, CFRange, CFStringCreateWithBytes, CFStringGetBytes, CFStringRef, kCFAllocatorDefault,
    kCFStringEncodingUTF8,
};

use super::acf_index::CFIndexConvertible as _;

// 获取字符串长度
#[inline]
fn get_len(cf_str_ref: CFStringRef) -> coreaudio_sys::CFIndex {
    unsafe { coreaudio_sys::CFStringGetLength(cf_str_ref) }
}

#[inline]
fn drop(cf_str_ref: CFStringRef) {
    unsafe { coreaudio_sys::CFRelease(cf_str_ref as coreaudio_sys::CFTypeRef) };
}

// 和macos的core audio 框架交互时，部分函数会获取 CFString 的所有权，
// 所以只提供crate内部函数，不做rust风格封装
// 会拷贝字节，如果不把所有权交给其它函数，那么调用者需要释放内存
pub(crate) fn create_cf_string_ref(string: &str) -> CFStringRef {
    unsafe {
        CFStringCreateWithBytes(
            kCFAllocatorDefault,
            string.as_ptr(),
            string.len().to_cfindex(),
            kCFStringEncodingUTF8,
            false as Boolean,
        )
    }
}

// to String of rust, and drop CFString
pub(crate) fn into_string_drop(cf_str_ref: CFStringRef) -> String {
    let string = into_string(cf_str_ref).into_owned();
    drop(cf_str_ref);
    string
}

// to String of rust
fn into_string(cf_str_ref: CFStringRef) -> Cow<'static, str> {
    unsafe {
        // Do this without allocating if we can get away with it
        let c_string =
            coreaudio_sys::CFStringGetCStringPtr(cf_str_ref, coreaudio_sys::kCFStringEncodingUTF8);
        if !c_string.is_null() {
            let c_str = CStr::from_ptr(c_string);
            Cow::Borrowed(str::from_utf8_unchecked(c_str.to_bytes()))
        } else {
            let len = get_len(cf_str_ref);

            // First, ask how big the buffer ought to be.
            let mut bytes_required: coreaudio_sys::CFIndex = 0;
            CFStringGetBytes(
                cf_str_ref,
                CFRange {
                    location: 0,
                    length: len,
                },
                kCFStringEncodingUTF8,
                0,
                false as Boolean,
                ptr::null_mut(),
                0,
                &mut bytes_required,
            );

            // Then, allocate the buffer and actually copy.
            let mut buffer = vec![b'\x00'; bytes_required as usize];

            let mut bytes_used = 0;
            let chars_written = CFStringGetBytes(
                cf_str_ref,
                CFRange {
                    location: 0,
                    length: len,
                },
                kCFStringEncodingUTF8,
                0,
                false as Boolean,
                buffer.as_mut_ptr(),
                buffer.len().to_cfindex(),
                &mut bytes_used,
            );
            assert_eq!(chars_written, len);
            // This is dangerous; we over-allocate and null-terminate the string (during
            // initialization).
            assert_eq!(bytes_used, buffer.len().to_cfindex());

            Cow::Owned(String::from_utf8_unchecked(buffer))
        }
    }
}
