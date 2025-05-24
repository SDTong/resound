//! provide CFNumber operate

use coreaudio_sys::CFIndex;

pub trait CFIndexConvertible {
    /// Always use this method to construct a `CFIndex` value. It performs bounds checking to
    /// ensure the value is in range.
    fn to_cfindex(self) -> CFIndex;
}

impl CFIndexConvertible for usize {
    #[inline]
    fn to_cfindex(self) -> CFIndex {
        let max_cfindex = CFIndex::max_value();
        if self > (max_cfindex as usize) {
            panic!("value out of range")
        }
        self as CFIndex
    }
}
