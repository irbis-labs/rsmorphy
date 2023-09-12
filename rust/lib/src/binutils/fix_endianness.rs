use byteorder::ByteOrder;

use super::ref_to_bytes_mut;

pub fn rewrite_u32<Order: ByteOrder>(v: &mut u32) {
    let copy = *v;
    Order::write_u32(unsafe { ref_to_bytes_mut(v) }, copy);
}

pub fn rewrite_u128<Order: ByteOrder>(v: &mut u128) {
    let copy = *v;
    Order::write_u128(unsafe { ref_to_bytes_mut(v) }, copy);
}
