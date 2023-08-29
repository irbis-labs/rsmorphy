use std::str::from_utf8_unchecked;

use smallvec::SmallVec;

use crate::dawg::dictionary::Dictionary;
use crate::dawg::guide::Guide;

#[derive(Debug, Clone)]
pub struct Completer<'a> {
    dict: &'a Dictionary,
    guide: &'a Guide,
    key: SmallVec<[u8; 64]>,
    index_stack: SmallVec<[u32; 64]>,
    last_index: u32,
}

impl<'a> Completer<'a> {
    pub(super) fn new(dict: &'a Dictionary, guide: &'a Guide, index: u32, prefix: &str) -> Self {
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

    #[allow(dead_code)]
    #[inline(always)]
    pub fn value(&self) -> u32 {
        self.dict.value(self.last_index)
    }

    /// Gets the next key
    pub fn next_key(&mut self) -> Option<&str> {
        let mut last_index = *self.index_stack.last()?;

        if self.last_index != self.dict.root {
            let child_label = self.child(last_index);
            if child_label != 0 {
                // Follows a transition to the first child.
                last_index = self.follow(child_label, last_index)?;
            } else {
                let mut sibling_label = 0;
                while sibling_label == 0 {
                    sibling_label = self.sibling(last_index);
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
    fn follow(&mut self, label: u8, index: u32) -> Option<u32> {
        let next_index = self.dict.follow_char(label, index)?;
        self.key.push(label);
        self.index_stack.push(next_index);
        Some(next_index)
    }

    fn find_terminal(&mut self, mut index: u32) -> Option<&str> {
        while !self.dict.has_value(index) {
            let child_label = self.child(index);
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

    pub fn key(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.key) }.trim_end()
    }

    #[inline(always)]
    fn child(&self, index: u32) -> u8 {
        self.guide.unit(index).child
    }

    #[inline(always)]
    fn sibling(&self, index: u32) -> u8 {
        self.guide.unit(index).sibling
    }
}
