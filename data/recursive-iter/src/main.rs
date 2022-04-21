pub struct Node {
    pub values: Vec<i32>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn values<'a>(&'a self) -> Box<dyn Iterator<Item = &i32> + 'a> {
        Box::new(
            self.values
                .iter()
                .chain(self.children.iter().map(|n| n.values()).flatten()),
        )
    }
}

fn main() {
    println!("Hello, world!");
}
