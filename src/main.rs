use std::io;


fn main() {
    Proposition();
}

fn Proposition() {

    let mut name = String::new();

    io::stdin()
        .read_line(&mut names)
        .expect("Failed to read line");

    println!("{names}");
}