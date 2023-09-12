use std::mem;
use std::slice;

pub unsafe fn ref_to_bytes_mut<T>(v: &mut T) -> &mut [u8] {
    let len = mem::size_of::<T>();
    slice::from_raw_parts_mut(v as *mut T as *mut u8, len)
}

pub unsafe fn ref_to_bytes<T>(v: &T) -> &[u8] {
    let len = mem::size_of::<T>();
    slice::from_raw_parts(v as *const T as *const u8, len)
}
