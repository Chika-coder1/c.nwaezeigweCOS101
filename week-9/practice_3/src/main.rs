use std::io::Write;
use std::fs;

fn main() {
    /*let 3rd_message = "This message was made to be deleted in practice_3, but I change my mind I wont use it";*/
    let file = std::fs::File::create("to_be_deleted.txt").expect("Create failed");
    fs::remove_file("to_be_deleted.txt").expect("could not remove file");
    println!("file is removed");
}
