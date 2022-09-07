#[derive(Debug)]
struct Proposition {
    name: String,
    value: f64,
}

trait Leaf {
    fn new(name: String) -> Self;
    fn print(&self);
}

impl Leaf for Proposition {
    fn new(name: String) -> Proposition {
        return Proposition { name, value: 0.5 };
    }

    fn print(&self) {
        println!("{}", self.name);
    }
}

fn main() {
    let mut messi: Proposition = Leaf::new(String::from("Messi"));
    messi.print();
}
