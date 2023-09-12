pub trait Suspend {
    type Suspended: Revive;

    fn suspend(&self) -> Self::Suspended;
}

pub trait Revive {
    type Revived<'d>;

    unsafe fn revive<'d>(&self, source_bin: &'d [u8]) -> Self::Revived<'d>;
}
