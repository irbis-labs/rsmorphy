#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ParadigmId(u16);

#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ParadigmIndex(u16);

impl ParadigmId {
    pub fn new<I>(id: I) -> Self
    where
        I: Into<u16>,
    {
        ParadigmId(id.into())
    }

    pub fn new_checked_u<I>(id: I) -> Self
    where
        I: Into<u128>,
    {
        let id = id.into();
        assert!(id <= ::std::u16::MAX.into());
        Self::new(id as u16)
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl From<u8> for ParadigmId {
    fn from(id: u8) -> Self {
        Self::new(id)
    }
}

impl From<u16> for ParadigmId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<u32> for ParadigmId {
    fn from(index: u32) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<u64> for ParadigmId {
    fn from(index: u64) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<u128> for ParadigmId {
    fn from(index: u128) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<usize> for ParadigmId {
    fn from(index: usize) -> Self {
        Self::new_checked_u(index as u128)
    }
}


impl ParadigmIndex {
    pub fn new<I>(index: I) -> Self
    where
        I: Into<u16>,
    {
        ParadigmIndex(index.into())
    }

    pub fn new_checked_u<I>(id: I) -> Self
    where
        I: Into<u128>,
    {
        let id = id.into();
        assert!(id <= ::std::u16::MAX.into());
        Self::new(id as u16)
    }

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn is_first(&self) -> bool {
        self.0 == 0
    }
}

impl From<u8> for ParadigmIndex {
    fn from(id: u8) -> Self {
        Self::new(id)
    }
}

impl From<u16> for ParadigmIndex {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<u32> for ParadigmIndex {
    fn from(index: u32) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<u64> for ParadigmIndex {
    fn from(index: u64) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<u128> for ParadigmIndex {
    fn from(index: u128) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<usize> for ParadigmIndex {
    fn from(index: usize) -> Self {
        Self::new_checked_u(index as u128)
    }
}
