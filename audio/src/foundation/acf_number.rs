//! provide CFNumber operate

use std::ffi::c_void;

use coreaudio_sys::{
    CFNumberCreate, CFNumberRef, CFNumberType, kCFAllocatorDefault, kCFNumberSInt32Type,
};

// 和macos的core audio 框架交互时，部分函数会获取 CFNumberRef 的所有权，
// 所以只提供crate内部函数，不做rust风格封装
// 如果不把所有权交给其它函数，那么调用者需要释放内存
pub(crate) fn create_cf_number_ref(value: i32) -> CFNumberRef {
    unsafe {
        CFNumberCreate(
            kCFAllocatorDefault,
            kCFNumberSInt32Type as CFNumberType,
            &value as *const i32 as *const c_void,
        )
    }
}
