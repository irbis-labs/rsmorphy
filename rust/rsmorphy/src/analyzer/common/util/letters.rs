#[derive(Debug, Clone, Copy)]
pub struct Letters {
    chars: &'static [char],
    letters: &'static [&'static str],
}

impl Letters {
    pub const fn new(chars: &'static [char], letters: &'static [&'static str]) -> Self {
        assert!(chars.len() == letters.len());
        Letters { chars, letters }
    }

    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        (0..self.chars.len()).all(|i| self.letters[i].chars().next().unwrap() == self.chars[i])
    }

    pub fn find(&self, letter: &str) -> Option<&'static str> {
        let mut it = letter.chars();

        let Some(letter) = it.next() else { None? };

        if it.next().is_some() {
            None?
        }

        let index = self.chars.iter().copied().position(|c| c == letter)?;

        Some(self.letters[index])
    }
}
