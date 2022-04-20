use std::fmt::Debug;

pub struct Bit<'a> {
    inner: &'a [u8],
    index: usize,
}

impl<'a> Bit<'a> {
    pub fn new(inner: &'a [u8], index: usize) -> Self {
        Self { inner, index }
    }

    fn get_bit(item: u8, offset: usize) -> u8 {
        item >> (7 - offset) & 1
    }

    fn to_u8(&self) -> u8 {
        let (index, offset) = (self.index / 8, self.index % 8);
        Bit::get_bit(self.inner[index], offset)
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

fn main() {
    let x = [0b_1010_0101u8];
    println!("x={:?}", x);

    let b: u8 = Bit::new(&x, 0).into();
    eprintln!("b_as_u8={:?}", b);

    let b: bool = Bit::new(&x, 0).into();
    eprintln!("b_as_bool={:?}", b);

    let xs: Vec<bool> = BitIterator::new(&x).collect();
    println!("xs={:?}", xs);
}
