use std::ops;

use smallvec::SmallVec;

use super::CompletionDawg;
use super::CompletionDawgSuspended;
use super::DawgValue;
use crate::cryostasis::Revive;
use crate::cryostasis::Suspend;

#[derive(Debug)]
pub struct DawgList<'d, V>
where
    V: DawgValue,
{
    list: SmallVec<[CompletionDawg<'d, V>; 4]>,
}

#[derive(Debug)]
pub struct DawgListSuspended<V>
where
    V: DawgValue,
{
    list: SmallVec<[CompletionDawgSuspended<V>; 4]>,
}

impl<'d, V> Suspend for DawgList<'d, V>
where
    V: DawgValue,
{
    type Suspended = DawgListSuspended<V>;

    fn suspend(&self) -> Self::Suspended {
        let list = self.list.iter().map(|d| d.suspend()).collect();
        DawgListSuspended { list }
    }
}

impl<V> Revive for DawgListSuspended<V>
where
    V: DawgValue,
{
    type Revived<'d> = DawgList<'d, V>;

    unsafe fn revive<'d>(&self, source_bin: &'d [u8]) -> DawgList<'d, V> {
        let list = self.list.iter().map(|d| d.revive(source_bin)).collect();
        DawgList { list }
    }
}

impl<'d, V> DawgList<'d, V>
where
    V: DawgValue,
{
    pub fn new(list: SmallVec<[CompletionDawg<'d, V>; 4]>) -> Self {
        DawgList { list }
    }
}

impl<'d, V> ops::Deref for DawgList<'d, V>
where
    V: DawgValue,
{
    type Target = [CompletionDawg<'d, V>];

    fn deref(&self) -> &Self::Target {
        self.list.deref()
    }
}

impl<V> ops::Deref for DawgListSuspended<V>
where
    V: DawgValue,
{
    type Target = [CompletionDawgSuspended<V>];

    fn deref(&self) -> &Self::Target {
        self.list.deref()
    }
}

impl<'d, V> Clone for DawgList<'d, V>
where
    V: DawgValue,
{
    fn clone(&self) -> Self {
        DawgList {
            list: self.list.clone(),
        }
    }
}
