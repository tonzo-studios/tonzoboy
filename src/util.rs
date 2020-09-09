/// Creates a word from two bytes
#[inline]
pub fn make_word(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) | (lsb as u16)
}

/// Return the LSB (least significant byte) of the given word
#[inline]
pub fn lsb(word: u16) -> u8 {
    (word & 0x00FF) as u8
}

/// Return the MSB (most significant byte) of the given word
#[inline]
pub fn msb(word: u16) -> u8 {
    (word >> 8) as u8
}

/// Swap upper and lower nibbles of the byte and return the result
#[inline]
pub fn swap(byte: u8) -> u8 {
    (byte << 4) | (byte >> 4)
}

#[inline]
pub fn rotate_left(byte: u8, amount: u8) -> u8 {
    (byte << amount) | (byte >> (8 - amount))
}

#[inline]
pub fn rotate_right(byte: u8, amount: u8) -> u8 {
    (byte >> amount) | (byte << (8 - amount))
}
