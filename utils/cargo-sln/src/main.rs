use cargo_toml::Manifest;
use std::fs::read;

fn own() {
    let m = Manifest::<toml::Value>::from_slice_with_metadata(&read("Cargo.toml").unwrap()).unwrap();
    let package = m.package();

    let authors = package.authors();
    println!("VLD::authors: {authors:?}");

    let description = package.description();
    println!("VLD::description: {description:?}");
}

fn main() {
    println!("Hello, world!");
    own();
}
