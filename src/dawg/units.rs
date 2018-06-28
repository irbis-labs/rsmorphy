pub const PRECISION_MASK: u32 = 0xFFFF_FFFF;

pub const OFFSET_MAX: u32 = 0x0020_0000; // 1 << 21
pub const IS_LEAF_BIT: u32 = 0x8000_0000; // 1 << 31
pub const HAS_LEAF_BIT: u32 = 0x0000_0100; // 1 << 8
pub const EXTENSION_BIT: u32 = 0x0000_0200; // 1 << 9

/// Check if a unit has a leaf as a child or not.
pub fn has_leaf(base: u32, mask: Option<u32>) -> bool {
    let mask = mask.unwrap_or(HAS_LEAF_BIT);
    base & mask != 0
}

/// Check if a unit corresponds to a leaf or not.
pub fn value(base: u32, mask: Option<u32>) -> u32 {
    let mask = mask.unwrap_or(IS_LEAF_BIT ^ PRECISION_MASK);
    base & mask
}

/// Read a label with a leaf flag from a non-leaf unit.
pub fn label(base: u32, mask: Option<u32>) -> u32 {
    let mask = mask.unwrap_or(IS_LEAF_BIT | 0x0000_00FF);
    base & mask
}

/// Read an offset to child units from a non-leaf unit.
pub fn offset(base: u32) -> u32 {
    (base >> 10) << ((base & EXTENSION_BIT) >> 6)
}
