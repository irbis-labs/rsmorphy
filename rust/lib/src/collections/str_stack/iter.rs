use crate::collections::str_stack::ref_::StrListRef;

#[derive(Debug, Clone)]
pub struct StrListIter<'a> {
    list: StrListRef<'a>,
    cursor: Option<(usize, usize)>,
}

impl<'a> StrListIter<'a> {
    pub fn new(list: StrListRef<'a>) -> Self {
        let cursor = if list.len() > 0 {
            Some((0, list.len() - 1))
        } else {
            None
        };
        StrListIter { list, cursor }
    }
}

impl<'a> Iterator for StrListIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let (begin, end) = self.cursor.take()?;

        if begin < end {
            let next_begin = begin + 1;
            self.cursor = Some((next_begin, end));
        }

        // SAFETY: index is checked above.
        Some(unsafe { self.list.get_unchecked(begin) })
    }
}

impl<'a> DoubleEndedIterator for StrListIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (begin, end) = self.cursor.take()?;

        if begin < end {
            let next_end = end - 1;
            self.cursor = Some((begin, next_end));
        }

        // SAFETY: index is checked above.
        Some(unsafe { self.list.get_unchecked(end) })
    }
}

impl<'a> ExactSizeIterator for StrListIter<'a> {
    fn len(&self) -> usize {
        self.cursor.map_or(0, |(begin, end)| end - begin + 1)
    }
}
