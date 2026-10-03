/*  .greet user***
    .display menu and tell user to pick using the codes***
    .Assign each code to a price***
    .Take users input
    .Add the users input together
    .check if the are eligible for a dicount and say so
    .display the price.*/

use std::io;
fn main() {

println!("Welcome customer!");
println!("Have a look at our menu!");
println!("p --- Poundo Yam / Edinkaiko Soup -------- #3_200");
println!("f --- Fried Rice & Chicken -------- #3_000");
println!("a --- Amala & Ewedu Soup -------- #2_500");
println!("e --- Eba & Ewedu Soup -------- #2_000");
println!("w --- White Rice & Stew -------- #2_500 ");

println!("To order input the meals code, which is the first letter of what you are ordering.");

let mut total_price = 0;

loop {
    let mut user_order = String::new();

    io::stdin()
        .read_line(&mut user_order)
        .expect("Invalid input");

    let trimmed = user_order.trim().to_lowercase();

    if trimmed == "done" || trimmed.is_empty() {
        break;
    }


    for item in user_order.trim().split(",") {
        let code = item.trim().to_lowercase();

        if code == "p" {
            total_price += 3_200;
        } else if code == "f" {
            total_price += 3_000;
        } else if code == "a" {
            total_price += 2_500;
        } else if code == "e" {
            total_price += 2_000;
        } else if code == "w" {
            total_price += 2_500;
        } else if code == "" {
            continue;
        } else {
            println!("Note: We don't have '{}' on the menu.", code);
        }
    }
            println!("Your total is: #{}", total_price);
            println!("Add more items or type 'done' to finish:");
  }
  println!("Final price: #{}", total_price);

  if total_price > 10_000 {
  let new_final_price = total_price - (total_price * 5) / 100;
    println!("You have a 5% discount. Your total is #{}", new_final_price);
  }          
else 
{
    println!("Your amount is #{}", total_price);
}
}