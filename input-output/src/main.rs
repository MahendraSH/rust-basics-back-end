use std::io;

fn main() {
    println!("Enter a a line ");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect(" can not read line");
    println!(" The line :-> {line}");
}
