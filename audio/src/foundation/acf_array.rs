//! provide CFArray operate

use std::ffi::c_void;

use coreaudio_sys::{CFArrayCreate, CFArrayRef, kCFAllocatorDefault, kCFTypeArrayCallBacks};

use super::acf_index::CFIndexConvertible as _;

// 和macos的core audio 框架交互时，部分函数会获取 CFArrayRef 的所有权，
// 所以只提供crate内部函数，不做rust风格封装
// 会拷贝字节，如果不把所有权交给其它函数，那么调用者需要释放内存
pub(crate) fn create_cf_array_ref<T: super::Ref>(elems: &[T]) -> CFArrayRef {
    unsafe {
        CFArrayCreate(
            kCFAllocatorDefault,
            elems.as_ptr() as *const *const c_void as *mut *const c_void,
            elems.len().to_cfindex(),
            &kCFTypeArrayCallBacks,
        )
    }
}
