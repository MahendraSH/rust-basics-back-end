use std::io;
fn main() {
    println!(" Guess a number");
    println!("Please input the Guess");
    let mut guess_number = String::new();
    io::stdin()
        .read_line(&mut guess_number)
        .expect(" Failed not able to read line");
    println!("guess_number = {guess_number}");
}
