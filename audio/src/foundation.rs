//! provide core foundation Framework operate

mod acf_array;
mod acf_index;
mod acf_number;
mod acf_string;

pub(crate) use acf_array::*;
pub(crate) use acf_number::*;
pub(crate) use acf_string::*;

// 标记 core foundtion 中的 xxRef
pub(crate) trait Ref {}
impl Ref for coreaudio_sys::CFNumberRef {}
