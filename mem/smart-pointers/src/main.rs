use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::sync::{Arc, Mutex};
use std::{thread, time};


// fn do_something<'a>(x: &'a u64) -> &'a u64 {
//     println!("{}", x);
//     x
// }

fn test_rc() {
    let pointer = Rc::new(1);

    {
        let second_pointer = pointer.clone();
        println!("{}", *second_pointer);
    }

    println!("{}", *pointer);
}

fn test_rc_mut() {
    let shared_string = Rc::new(RefCell::new("Hello ".to_string()));

    {
        let mut hello_world: RefMut<String> = shared_string.borrow_mut();
        hello_world.push_str("world");
    }

    println!("{}", shared_string.take());
}

fn test_arc() {
    let pointer = Arc::new(5);

    let second_pointer = pointer.clone();
    thread::spawn(move || {
        println!("{}", *second_pointer);
    });

    thread::sleep(time::Duration::from_secs(1));

    println!("{}", *pointer);
}

fn test_arc_mut() {
    let pointer = Arc::new(Mutex::new(5));

    let second_pointer = pointer.clone();
    thread::spawn(move || {
        let mut mutable_pointer = second_pointer.lock().unwrap();
        *mutable_pointer = 1;
    });

    thread::sleep(time::Duration::from_secs(1));

    let one = pointer.lock().unwrap();
    println!("{}", one);
}

fn main() {
    test_rc();
    test_rc_mut();
    test_arc();
    test_arc_mut();
}
