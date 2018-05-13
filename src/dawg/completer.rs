use super::dictionary::Dictionary;
use super::guide::Guide;


#[derive(Debug, Clone)]
pub struct Completer<'a> {
    pub dict: &'a Dictionary,
    pub guide: &'a Guide,
    pub last_index: u32,
    pub key: Vec<u8>,
    pub index_stack: Vec<u32>
}


impl <'a> Completer<'a> {
    pub fn new(dict: &'a Dictionary, guide: &'a Guide, index: u32, prefix: &[u8]) -> Self {
        Completer {
            dict,
            guide,
            // unimplemented is unimplemented in the origin
            last_index: if guide.units.is_empty() { unimplemented!() } else { dict.root },
            key: prefix.to_owned(),
            index_stack: if guide.units.is_empty() { vec![] } else { vec![index] },
        }
    }

    pub fn value(&self) -> u32 {
        self.dict.value(self.last_index)
    }

    /// Gets the next key
    pub fn prepare_next(&mut self) -> bool {
        if self.index_stack.is_empty() {
            return false;
        }
        let mut index = self.index_stack[self.index_stack.len() - 1];

        if self.last_index != self.dict.root {
            let child_label = self.guide.units[index as usize].child;
            if child_label != 0 {
                // Follows a transition to the first child.
                index = match self.follow(child_label, index) {
                    Some(v) => v,
                    None => return false,
                };
            } else {
                'l: loop {
                    let sibling_label = self.guide.units[index as usize].sibling;
                    // Moves to the previous node.
                    if !self.key.is_empty() {
                        self.key.pop();
                    }
                    self.index_stack.pop();
                    if self.index_stack.is_empty() {
                        return false;
                    }
                    index = self.index_stack[self.index_stack.len() - 1];
                    if sibling_label != 0 {
                        // Follows a transition to the next sibling.
                        index = match self.follow(sibling_label, index) {
                            Some(v) => v,
                            None => return false,
                        };
                        break 'l;
                    }
                }
            }
        }
        self.find_terminal(index)
    }

    fn follow(&mut self, label: u8, index: u32) -> Option<u32> {
        self.dict.follow_char(label, index).map(|next_index| {
            self.key.push(label);
            self.index_stack.push(next_index);
            next_index
        })
    }

    fn find_terminal(&mut self, index: u32) -> bool {
        let mut index = index;
        while !self.dict.has_value(index) {
            let label = self.guide.units[index as usize].child;
            index = match self.dict.follow_char(label, index) {
                Some(v) => v,
                None => return false,
            };
            self.key.push(label);
            self.index_stack.push(index);
            trace!(r#"Completer::find_terminal() "#);
            trace!(r#" key: {}, stack = {:?} "#,
                     String::from_utf8(self.key.clone()).unwrap(),
                     self.index_stack);

        }
        self.last_index = index;
        true
    }
}
