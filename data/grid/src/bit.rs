use std::fmt::Debug;

pub struct Bit<'a> {
    inner: &'a [u8],
    index: usize,
}

impl<'a> Bit<'a> {
    pub fn new(inner: &'a [u8], index: usize) -> Self {
        Self { inner, index }
    }

    fn to_u8(&self) -> u8 {
        let (index, offset) = (self.index / 8, self.index % 8);
        let b: u8 = self.inner[index] >> (7 - offset);
        b % 2
    }
}

impl<'a> Debug for Bit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?} - {}", self.inner, self.index)
    }
}

impl<'a> From<Bit<'a>> for bool {
    fn from(bit: Bit<'a>) -> Self {
        bit.to_u8() == 1
    }
}

impl<'a> From<Bit<'a>> for u8 {
    fn from(bit: Bit<'a>) -> Self {
        bit.to_u8()
    }
}

pub struct BitIterator<'a>(Bit<'a>);

impl<'a> BitIterator<'a> {
    pub fn new(inner: &'a [u8]) -> BitIterator<'a> {
        BitIterator(Bit::new(inner, 0))
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.index >= self.0.inner.len() * 8 {
            return None;
        }

        let b: bool = self.0.to_u8() == 1;
        self.0.index += 1;
        Some(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bit() {
        let x = [0b11110000u8];
        let bit = Bit::new(&x, 0);
        assert_eq!(bit.index, 0);
    }
}
