use std::io::Write;
use std::io::Read;

fn main() {
    let message = "This is a file I created for the sake of practice 2\n";
    
    let mut file = std::fs::File::create("welcome_message.txt").expect("Smth wrong with");
    file.write_all(message.as_bytes()).expect("Write failed");
    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents );
}
