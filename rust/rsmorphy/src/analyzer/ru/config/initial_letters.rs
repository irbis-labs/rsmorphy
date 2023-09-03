use crate::analyzer::common::util::Letters;

/// letters initials can start with.
pub static INITIAL_LETTERS: Letters = Letters::new(
    &[
        'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р',
        'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Э', 'Ю', 'Я',
    ],
    &[
        "А", "Б", "В", "Г", "Д", "Е", "Ё", "Ж", "З", "И", "Й", "К", "Л", "М", "Н", "О", "П", "Р",
        "С", "Т", "У", "Ф", "Х", "Ц", "Ч", "Ш", "Щ", "Э", "Ю", "Я",
    ],
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_letters_is_valid() {
        assert!(INITIAL_LETTERS.is_valid());
    }
}
