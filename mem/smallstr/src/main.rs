use smartstring::{Compact, SmartString};

fn main() {
    let smart = SmartString::<Compact>::from("hello world");
    let smart_meta = &smart as *const _;
    let smart_data = &smart.as_bytes()[0] as *const _;
    dbg!((smart_meta, smart_data));

    let stand = String::from("hello world");
    let stand_meta = &stand as *const _;
    let stand_data = &stand.as_bytes()[0] as *const _;
    dbg!((stand_meta, stand_data));
}