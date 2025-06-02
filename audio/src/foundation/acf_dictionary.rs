//! provide CFDictionary operate

use coreaudio_sys::{CFDictionaryCreate, CFDictionaryRef, kCFAllocatorDefault};

use crate::{
    Ref,
    aoerror::{AudioError, Result},
};

use super::acf_index::CFIndexConvertible;

// 和macos的 core audio 框架交互时，部分函数会获取 CFDictionaryRef 的所有权，
// 所以只提供 crate 内部函数，不做rust风格封装
// 如果不把所有权交给其它函数，那么调用者需要释放内存
pub(crate) fn create_cf_dictionary_ref<T: Ref, U: Ref>(
    keys: &mut [T],
    values: &mut [U],
) -> CFDictionaryRef {
    try_create_cf_dictionay_ref(keys, values).unwrap()
}

fn try_create_cf_dictionay_ref<T: Ref, U: Ref>(
    keys: &mut [T],
    values: &mut [U],
) -> Result<CFDictionaryRef> {
    if keys.len() != values.len() {
        return Err(AudioError::with_msg(
            "the lengths of keys and values are different",
        ));
    }
    let cf_dictionary_ref = unsafe {
        CFDictionaryCreate(
            kCFAllocatorDefault,
            keys.as_mut_ptr() as *mut *const ::std::os::raw::c_void,
            values.as_mut_ptr() as *mut *const ::std::os::raw::c_void,
            keys.len().to_cfindex(),
            std::ptr::null(),
            std::ptr::null(),
        )
    };

    if cf_dictionary_ref.is_null() {
        Err(AudioError::with_msg("create CFDictionaryRef fail"))
    } else {
        Ok(cf_dictionary_ref)
    }
}
