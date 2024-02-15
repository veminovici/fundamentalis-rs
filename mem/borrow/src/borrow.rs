use std::{
    borrow::{Borrow, BorrowMut},
    fmt::Display,
};

fn borrow<T>(value: T)
where
    T: Borrow<i32> + Display,
{
    println!("value is a borrowed: {value}");
}

fn borrow_mut<T>(value: T)
where
    T: BorrowMut<i32> + Display,
{
    println!("value is a borrowed_mut: {value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow() {
        let mut i = 5;

        borrow(i);
        borrow(&i);
        borrow(&mut i);
    }

    #[test]
    fn test_borrow_mut() {
        let mut i = 5;

        borrow_mut(i);
        borrow_mut(&mut i);
    }
}
