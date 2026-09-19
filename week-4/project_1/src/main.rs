use std::io;
fn main() {
    // Given the values of a, b and c, find the roots of a quadratic equation with a Rust program. Read all three values from the keyboard.
    // ax^2 + bx + c = 0
println!("So you want to solve a quadratic equation huh?");
println!("You got guts my friend." );
println!("Lets not waste any time then, put in yer values.");
println!("Whats yer 'a'? ");
    

    //input your a value.

    let mut a = String::new();


    io::stdin()
        .read_line(&mut a )
        .expect("Yer inputs far from valid matey");

    let a:i64 = a
        .trim()
        .parse()
        .expect("Yer value fer 'a' needs some work laddey");


    //now its time for b.

    println!("Come on then whats yer 'b'?");

    let mut b = String::new();

    io::stdin()
        .read_line(&mut b)
        .expect("Yer input for 'b' not right matey");

    let b:i64 = b
        .trim()
        .parse()
        .expect("Try something else fer yer 'b' input");


    //now its time for c.

    println!("We're almost there laddy whats yer 'c'?");

    let mut c = String::new();

    io::stdin()
        .read_line(&mut c)
        .expect("Yer inputs wrong ye land-lubber!");

    let c:i64 = c
        .trim()
        .parse()
        .expect("Yer value for 'c' is wrong, thinK again matey");

        // Now it's time to calculate the discriminant
    let discriminant = (b as f64).powi(2) - 4.0 * (a as f64) * (c as f64);

    if discriminant < 0.0 {
        println!("Blimey! Yer discriminant be negative ({}), which means yer roots be imaginary numbers!", discriminant);
        println!("No real treasure to be found in these waters, matey!");
    } else {
        let root1 = (-(b as f64) + discriminant.sqrt()) / (2.0 * (a as f64));
        let root2 = (-(b as f64) - discriminant.sqrt()) / (2.0 * (a as f64));

        // Giving the answers
        println!("The roots of yer Quadratic equation be as follows, {} and {}", root1, root2);

        println!("Well done friend you've done, you've conquered quadratic equations, and next up... THE SEVEN SEAS!!!. HAHAHAHAHAHAHA.");

    }
}