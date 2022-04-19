mod bit;

use bit::*;

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
