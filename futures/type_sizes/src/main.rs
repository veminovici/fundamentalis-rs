use std::mem::size_of;
trait SomeTrait {}

fn main() {
    println!("======== The size of different pointers in Rust: ========");
    println!("&dyn Trait:-----{}", size_of::<&dyn SomeTrait>());
    println!("&[&dyn Trait]:--{}", size_of::<&[&dyn SomeTrait]>());
    println!("Box<Trait>:-----{}", size_of::<Box<dyn SomeTrait>>());
    println!("&i32:-----------{}", size_of::<&i32>());
    println!("&[i32]:---------{}", size_of::<&[i32]>());
    println!("Box<i32>:-------{}", size_of::<Box<i32>>());
    println!("&Box<i32>:------{}", size_of::<&Box<i32>>());
    println!("[&dyn Trait;4]:-{}", size_of::<[&dyn SomeTrait; 4]>());
    println!("[i32;4]:--------{}", size_of::<[i32; 4]>());
}
