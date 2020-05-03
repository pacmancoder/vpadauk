use super::Word;

pub const fn is_valid_opcode(value: Word) -> bool {
    const WORD_BITS: usize = 13;
    const MAX_PDK13_WORD_VALUE: u16 = (1u16 << WORD_BITS as u16) - 1;
    value <= MAX_PDK13_WORD_VALUE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_opcode_word() {
        assert!(is_valid_opcode(0b0000000000000000));
        assert!(is_valid_opcode(0b0001111111111111));
    }

    #[test]
    fn invalid_opcode_word() {
        assert!(!is_valid_opcode(0b0010000000000000));
    }
}
