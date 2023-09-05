#[derive(Debug, Clone, Copy)]
pub struct Letters {
    chars: &'static [char],
}

impl Letters {
    pub const fn new(chars: &'static [char]) -> Self {
        Letters { chars }
    }

    pub fn find(&self, letter: &str) -> Option<char> {
        let mut it = letter.chars();

        let Some(letter) = it.next() else { None? };

        if it.next().is_some() {
            None?
        }

        self.chars.iter().copied().find(|&c| c == letter)
    }
}
