extern crate libc;

#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

extern "C" {
    pub static MY_CONSTANT: i32;
}

#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
