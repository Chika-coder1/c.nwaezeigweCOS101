use std::io;

fn main() {
    println!("How many years of experience do you have?");
    let mut experience = String::new();


    io::stdin()
        .read_line(&mut experience )
        .expect("Your input is invalid");

    let experience:i64 = experience
        .trim()
        .parse()
        .expect("Put a proper value for age");


    if experience >= 40
    {
        println!("Your pay is 1_560_000");
    }
    else if experience >= 30 && experience <= 39
    {
        println!("Your pay is 1_480_00");
    }
    else if experience  >= 1 && experience <= 28
    {
        println!("Your pay is 1_300_000");
    }
    else 
    {
        println!("No experience, so your pay is 100_000");
    }


}
