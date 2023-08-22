use crate::dawg::dictionary::Dictionary;
use crate::dawg::guide::Guide;

#[derive(Debug, Clone)]
pub struct Completer<'a> {
    dict: &'a Dictionary,
    guide: &'a Guide,
    last_index: u32,
    key: Vec<u8>,
    index_stack: Vec<u32>,
}

impl<'a> Completer<'a> {
    pub fn new<Prefix>(dict: &'a Dictionary, guide: &'a Guide, index: u32, prefix: Prefix) -> Self
    where
        Prefix: Into<Vec<u8>>,
    {
        let key = prefix.into();
        let (index_stack, last_index) = if guide.units.is_empty() {
            // FIXME unimplemented in the PyMorphy2; should it be removed?
            unimplemented!()
        } else {
            (vec![index], dict.root)
        };

        Completer {
            dict,
            guide,
            last_index,
            key,
            index_stack,
        }
    }

    pub fn value(&self) -> u32 {
        self.dict.value(self.last_index)
    }

    /// Gets the next key
    pub fn next_key(&mut self) -> Option<&str> {
        let mut last_index = *self.index_stack.last()?;

        if self.last_index != self.dict.root {
            let entry = self.guide.units[last_index as usize];
            let (child_label, mut sibling_label) = (entry.child, entry.sibling);
            if child_label != 0 {
                // Follows a transition to the first child.
                last_index = self.follow(child_label, last_index)?;
            } else {
                while sibling_label == 0 {
                    sibling_label = self.guide.units[last_index as usize].sibling;
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
            let child_label = self.guide.units[index as usize].child;
            index = self.follow(child_label, index)?;
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
        unsafe { ::std::str::from_utf8_unchecked(&self.key) }.trim_end()
    }
}
