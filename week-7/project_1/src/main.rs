use std::io;

fn trapezium_area() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter height:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let h: f64 = input1.trim().parse().expect("Please type a number");

    println!("Enter base 1:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let base1: f64 = input2.trim().parse().expect("Please type a number");

    println!("Enter base 2:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let base2: f64 = input3.trim().parse().expect("Please type a number");

    let t_area = h / 2.0 * (base1 + base2);
    println!("The area of your trapezium is {}.", t_area);
}

fn rhombus_area() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let diagonal1: f64 = input1.trim().parse().expect("Please type a number");

    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let diagonal2: f64 = input2.trim().parse().expect("Please type a number");

    let r_area = 0.5 * diagonal1 * diagonal2;
    println!("The area of your rhombus is {}.", r_area);
}

fn parallelogram_area() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let base: f64 = input1.trim().parse().expect("Please type a number");

    println!("Enter altitude:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let altitude: f64 = input2.trim().parse().expect("Please type a number");

    let p_area = base * altitude;
    println!("The area of your parallelogram is {}.", p_area);
}

fn surface_area_cube() {
    let mut input1 = String::new();

    println!("Enter side length:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let side: f64 = input1.trim().parse().expect("Please type a number");

    let c_area = 6.0 * side * side;
    println!("The surface area of your cube is {}.", c_area);
}

fn cylinder_volume() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let radius: f64 = input1.trim().parse().expect("Please type a number");

    println!("Enter height:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let height: f64 = input2.trim().parse().expect("Please type a number");

    let pi = std::f64::consts::PI;
    let c_volume = pi * radius * radius * height;
    println!("The volume of your cylinder is {}.", c_volume);
}

fn main() {
    println!("The list of possible calculations are:");
    println!("  T - Area of Trapezium");
    println!("  R - Area of Rhombus");
    println!("  P - Area of Parallelogram");
    println!("  C - Surface Area of Cube");
    println!("  V - Volume of Cylinder");

    println!("What are you calculating today?\nCalculate by inputting a letter code.");

    let mut user_response = String::new();
    io::stdin()
        .read_line(&mut user_response)
        .expect("Failed to read input");
    
    let choice = user_response.trim();

    if choice == "T" {
        trapezium_area();
    } else if choice == "R" {
        rhombus_area();
    } else if choice == "P" {
        parallelogram_area();
    } else if choice == "C" {
        surface_area_cube();
    } else if choice == "V" {
        cylinder_volume();
    } else {
        println!("Invalid option selected.");
    }
}