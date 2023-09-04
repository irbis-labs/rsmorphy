use std::fmt;
use std::iter::FromIterator;
use std::ops;

use crate::model::gram::gram::Gram;

#[derive(Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct GramSet {
    set: u128,
}

impl fmt::Debug for GramSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GramSet")
            .field("set", &format_args!("{:#b}", self.set))
            .finish()
    }
}

impl GramSet {
    #[inline(always)]
    pub fn empty() -> Self {
        Self::default()
    }

    #[inline(always)]
    fn from_set(set: u128) -> Self {
        Self { set }
    }

    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.set == 0
    }

    #[inline(always)]
    pub fn len(self) -> usize {
        self.set.count_ones() as usize
    }

    #[inline(always)]
    pub fn iter(self) -> GramSetIter {
        Self::into_iter(self)
    }

    #[inline(always)]
    pub fn insert(&mut self, gram: Gram) {
        let other = GramSet::from(gram);
        self.set |= other.set;
    }

    #[inline(always)]
    pub fn remove(&mut self, gram: Gram) {
        let other = gram.as_set();
        self.set &= !other.set;
    }

    /// Returns true if the set contains a value.
    #[inline(always)]
    pub fn contains(self, gram: Gram) -> bool {
        let other = GramSet::from(gram);
        self.set & other.set != 0
    }

    /// Returns true if self has no elements in common with other. This is equivalent to checking
    /// for an empty intersection.
    #[inline(always)]
    pub fn is_disjoint(self, other: Self) -> bool {
        (self & other).is_empty()
    }

    /// Returns true if the set has at least one element in common with other.
    #[inline(always)]
    pub fn is_intersect(self, other: Self) -> bool {
        !(self & other).is_empty()
    }

    /// Returns true if the set is a subset of another, i.e., other contains at least
    /// all the values in self.
    #[inline(always)]
    pub fn is_subset(self, other: Self) -> bool {
        self & other == self
    }

    /// Returns true if the set is a superset of another, i.e., self contains at least all
    /// the values in other.
    #[inline(always)]
    pub fn is_superset(self, other: Self) -> bool {
        self & other == other
    }

    #[inline(always)]
    pub fn union(self, other: Self) -> Self {
        self | other
    }

    #[inline(always)]
    pub fn intersection(self, other: Self) -> Self {
        self & other
    }

    #[inline(always)]
    pub fn difference(self, other: Self) -> Self {
        self - other
    }

    #[inline(always)]
    pub fn symmetric_difference(self, other: Self) -> Self {
        self ^ other
    }
}

impl From<Gram> for GramSet {
    #[inline(always)]
    fn from(gram: Gram) -> Self {
        let set = 1 << gram.id();
        GramSet { set }
    }
}

impl FromIterator<Gram> for GramSet {
    #[inline(always)]
    fn from_iter<T: IntoIterator<Item = Gram>>(iter: T) -> Self {
        let mut set = GramSet::empty();
        for gram in iter {
            set.insert(gram);
        }
        set
    }
}

impl ops::Not for GramSet {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self::from_set(!self.set)
    }
}

impl ops::BitOr for GramSet {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, other: Self) -> Self::Output {
        Self::from_set(self.set | other.set)
    }
}

impl ops::BitOrAssign for GramSet {
    #[inline(always)]
    fn bitor_assign(&mut self, other: Self) {
        self.set |= other.set;
    }
}

impl ops::BitAnd for GramSet {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, other: Self) -> Self::Output {
        Self::from_set(self.set & other.set)
    }
}

impl ops::BitAndAssign for GramSet {
    #[inline(always)]
    fn bitand_assign(&mut self, other: Self) {
        self.set &= other.set;
    }
}

impl ops::Sub for GramSet {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self::from_set(self.set & !other.set)
    }
}

impl ops::SubAssign for GramSet {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.set &= !other.set;
    }
}

impl ops::BitXor for GramSet {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, other: Self) -> Self::Output {
        Self::from_set(self.set ^ other.set)
    }
}

impl ops::BitXorAssign for GramSet {
    #[inline(always)]
    fn bitxor_assign(&mut self, other: Self) {
        self.set ^= other.set;
    }
}

impl IntoIterator for GramSet {
    type Item = Gram;
    type IntoIter = GramSetIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        GramSetIter::new(self.set)
    }
}

#[derive(Debug, Clone)]
pub struct GramSetIter {
    rest: u128,
    next: Option<u8>,
}

impl GramSetIter {
    #[inline(always)]
    fn new(set: u128) -> Self {
        let (rest, next) = Self::get_next(set >> 1, 0);
        Self { rest, next }
    }

    #[inline(always)]
    fn get_next(rest: u128, current: u8) -> (u128, Option<u8>) {
        // Если rest == 0, то next == None.
        if rest == 0 {
            return (rest, None);
        }
        // Находим следующий бит, который равен 1.
        let next = rest.trailing_zeros() as u8 + 1;
        debug_assert!(
            current + next < 128,
            "Next gram id is out of range 1..128: {} ({})",
            current + next,
            next
        );
        (rest >> next, Some(current + next))
    }
}

impl Iterator for GramSetIter {
    type Item = Gram;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        let (rest, next) = Self::get_next(self.rest, current);
        self.rest = rest;
        self.next = next;
        Some(unsafe { Gram::new_unchecked(current) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gram_set_iter() {
        let set = GramSet::from(Gram::new(1));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Gram::new(1)));
        assert_eq!(iter.next(), None);

        let set = GramSet::from(Gram::new(5));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Gram::new(5)));
        assert_eq!(iter.next(), None);

        let set = GramSet::from(Gram::new(127));
        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(Gram::new(127)));
        assert_eq!(iter.next(), None);
    }
}
