use crate::data_struct::string_list::array_ref::StringArrayRef;

#[derive(Debug, Clone)]
pub struct StringArrayIter<'a> {
    array: StringArrayRef<'a>,
    cursor: Option<(usize, usize)>,
}

impl<'a> StringArrayIter<'a> {
    pub fn new(array: StringArrayRef<'a>) -> Self {
        let cursor = if array.len() > 0 {
            Some((0, array.len() - 1))
        } else {
            None
        };
        StringArrayIter { array, cursor }
    }
}

impl<'a> Iterator for StringArrayIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let (begin, end) = self.cursor.take()?;

        if begin < end {
            let next_begin = begin + 1;
            self.cursor = Some((next_begin, end));
        }

        // SAFETY: index is checked above.
        Some(unsafe { self.array.get_unchecked(begin) })
    }
}

impl<'a> DoubleEndedIterator for StringArrayIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (begin, end) = self.cursor.take()?;

        if begin < end {
            let next_end = end - 1;
            self.cursor = Some((begin, next_end));
        }

        // SAFETY: index is checked above.
        Some(unsafe { self.array.get_unchecked(end) })
    }
}

impl<'a> ExactSizeIterator for StringArrayIter<'a> {
    fn len(&self) -> usize {
        self.cursor.map_or(0, |(begin, end)| end - begin + 1)
    }
}
