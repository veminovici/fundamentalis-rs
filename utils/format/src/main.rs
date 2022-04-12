fn test_format() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}

fn test_float_as_integer() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };

    println!("{}", b);
    assert_eq!(a, b);
}

fn test_int_past_its_range() {
    let mut i: u16 = 0;
    print!("{}..", i);

    loop {
        i += 1000;
        print!("{}..", i);

        if i % 10000 == 0 {
            print!("\n");
        }
    }
}

fn test_endian() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { std::mem::transmute(big_endian)};
    let b: i32 = unsafe { std::mem::transmute(little_endian)};

    println!("{} vs {}", a, b);
}

fn main() {
    // test_format();
    // test_float_as_integer();
    // test_int_past_its_range();
    test_endian();
}
