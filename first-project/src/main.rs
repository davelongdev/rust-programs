use std::io;

fn main() {
    let mut my_string: String = String::new();
    
    io::stdin()
        .read_line(&mut my_string)
        .expect("you gave me a bad value");

    println!("you typed: {}", my_string);
}
