use std::mem;

// TODO return error instead of panic
// TODO check alignment
pub unsafe fn bytes_to_ref<T>(bytes: &[u8]) -> &T {
    let size = mem::size_of::<T>();
    assert!(
        bytes.len() >= size,
        "assert bytes.len(): {} >= struct size: {} ",
        bytes.len(),
        size
    );
    &*(bytes as *const _ as *const T)
}
