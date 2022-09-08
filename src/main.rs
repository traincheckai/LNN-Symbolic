// Structs
#[derive(Debug)]
struct Proposition {
    name: String,
    value: f64,
}

// Traits
trait Leaf {
    fn new(name: String) -> Self;
    fn print(&self);
    fn edit(&mut self);
}

// Implementations
impl Leaf for Proposition {
    fn new(name: String) -> Proposition {
        return Proposition { name, value: 1.0 };
    }

    fn print(&self) {
        println!("{}, {}", self.name, self.value);
    }

    fn edit(&mut self) {
        let new_val = 0.5;
        self.value = new_val;
    }
}

fn main() {
    let mut messi: Proposition = Leaf::new(String::from("Messi"));
    messi.print();
    messi.edit();
    messi.print()
}
