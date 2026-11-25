use std::io::Read;
use std::io;

fn admin(){
    let mut file = std::fs::File::open("globacom_db.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn pm(){
let mut file = std::fs::File::open("projects_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn employee(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn customer(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn vendor(){
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}

fn main(){

    println!("What postion do you hold?;
        Administrator   -   a
        Project Manager -   p
        Employee    -       e
        Customer    -       c
        Vendor      -       v");

println!("input a,p,e,c,v");
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read input");

if input == "a" {admin();}
else if input == "p" {pm();}
else if input == "e" {employee();}
else if input == "c" {customer();}
else {vendor();}

}