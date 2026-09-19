use std::io;

fn main()
{
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Input your value for a: ");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:f16 = a.trim().parse().expect("Not a valid number");

    println!("Input your value for b: ");
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let b:f16 = b.trim().parse().expect("Not a valid number");

    println!("Input your value for c: ");
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let c:f16 = c.trim().parse().expect("Not a valid number");

    let mut d = b*b - 4.0*a*c;
    d = d.sqrt();

    if d > 0
    {
        println!();
    }
    else if {
        unimplemented!();
    }
    else if{}

}