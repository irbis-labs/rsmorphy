use std::str::from_utf8_unchecked;

use smallvec::SmallVec;

use crate::dawg::dict::DawgDictionary;
use crate::dawg::dict_entry::DictIndex;
use crate::dawg::guide::DawgGuide;

#[derive(Debug, Clone)]
pub struct Completer<'a> {
    dict: &'a DawgDictionary,
    guide: &'a DawgGuide,
    key: SmallVec<[u8; 64]>,
    index_stack: SmallVec<[DictIndex; 64]>,
    last_index: DictIndex,
}

impl<'a> Completer<'a> {
    pub(super) fn new(
        dict: &'a DawgDictionary,
        guide: &'a DawgGuide,
        index: DictIndex,
        prefix: &str,
    ) -> Self {
        let key = prefix.as_bytes().into();
        let index_stack = [index].into_iter().collect();
        let last_index = index;

        Completer {
            dict,
            guide,
            last_index,
            key,
            index_stack,
        }
    }

    // #[inline(always)]
    // pub fn value(&self) -> DictValue {
    //     self.dict.value(self.last_index)
    // }

    #[inline(always)]
    fn key(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.key) }.trim_end()
    }

    /// Gets the next key
    pub(in crate::dawg) fn next_key(&mut self) -> Option<&str> {
        let mut last_index = *self.index_stack.last()?;

        if self.last_index != DawgDictionary::ROOT_INDEX {
            let child_label = self.guide[last_index].child;
            if child_label != 0 {
                // Follows a transition to the first child.
                last_index = self.follow(child_label, last_index)?;
            } else {
                let mut sibling_label = 0;
                while sibling_label == 0 {
                    sibling_label = self.guide[last_index].sibling;
                    // Moves to the previous node.
                    self.key.pop();
                    self.index_stack.pop();
                    last_index = *self.index_stack.last()?;
                }
                // Follows a transition to the next sibling.
                last_index = self.follow(sibling_label, last_index)?;
            }
        }
        self.find_terminal(last_index)
    }

    /// Looks for a next index and pushes it with the label on the stack.
    fn follow(&mut self, label: u8, index: DictIndex) -> Option<DictIndex> {
        let next_index = self.dict.follow_byte(label.into(), index)?;
        self.index_stack.push(next_index);
        self.key.push(label);
        Some(next_index)
    }

    fn find_terminal(&mut self, mut index: DictIndex) -> Option<&str> {
        while !self.dict[index].has_leaf() {
            let child_label = self.guide[index].child;
            index = self.follow(child_label, index)?;

            #[cfg(feature = "trace_dawg")]
            log::trace!(
                r#"Completer::find_terminal() index: {:8x?}, key: {:?}, stack = {:x?} "#,
                index,
                self.key(),
                self.index_stack
            );
        }

        self.last_index = index;
        Some(self.key())
    }
}
