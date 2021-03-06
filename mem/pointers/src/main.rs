use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;
    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size: {:?} bytes", size_of::<usize>());
    println!("  value: {:?}", a);

    println!("b (a reference to B)");
    println!("  location: {:p}", &b);
    println!("  size: {:?} bytes", size_of::<&[u8; 10]>());
    println!("  points to: {:p}", b);

    println!("c (a box for C)");
    println!("  location: {:p}", &c);
    println!("  size: {:?} bytes", size_of::<Box<[u8]>>());
    println!("  points to: {:p}", c);

    println!("B (an array of 10 bytes)");
    println!("  location: {:p}", &B);
    println!("  size: {:?} bytes", size_of::<[u8; 10]>());
    println!("  value: {:?}", B);

    println!("C (an array of 11 bytes)");
    println!("  location: {:p}", &C);
    println!("  size: {:?} bytes", size_of::<[u8; 11]>());
    println!("  values: {:?}", C);

    let a_ptr = &a as *const usize;
    let a_addr: usize = unsafe {
        std::mem::transmute(a_ptr)
    };

    unsafe {
        let new_ptr = a_ptr.offset(7);
        println!("a: {} ({:p}) 0x{:x} {:p}", a, a_ptr, a_addr + 7, new_ptr);
    }
}
