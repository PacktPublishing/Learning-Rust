use std::io;
fn main() {    
    let reader: io::Stdin = io::stdin();
    let mut input_text: String = String::new();

    reader.read_line(&mut input_text).expect("Reading failed");
    println!("Read {}", input_text);
}
