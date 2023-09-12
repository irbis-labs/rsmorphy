use std::ops;

use super::BinPack;

#[derive(Debug)]
#[repr(transparent)]
pub struct BinPackRef {
    data: [u8],
}

impl BinPackRef {
    #[inline(always)]
    pub fn new(data: &BinPack) -> &Self {
        let data = data.as_bytes();
        unsafe { Self::from_aligned_unchecked(data) }
    }

    #[inline(always)]
    pub unsafe fn from_aligned_unchecked(data: &[u8]) -> &Self {
        unsafe { &*(data as *const [u8] as *const Self) }
    }
}

impl ops::Deref for BinPackRef {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl ToOwned for BinPackRef {
    type Owned = BinPack;

    fn to_owned(&self) -> Self::Owned {
        BinPack::from(&self.data)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::ops::Deref;

    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(mem::size_of::<&[u8]>(), mem::size_of::<&BinPackRef>());
    }

    #[test]
    fn test_cow() {
        use std::borrow::Borrow;

        let data = [1_u8, 2, 3];
        let pack = BinPack::from(&data[..]);
        let cow: Cow<BinPackRef> = Cow::Owned(pack);
        let ref_: &BinPackRef = cow.borrow();
        assert_eq!(ref_.deref(), &data[..]);
    }
}
