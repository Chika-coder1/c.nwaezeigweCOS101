use std::io::stdin;

fn main() {
    let mut developers: Vec<(String, u32)> = Vec::new();
    let mut best = (String::new(), 0);

    loop {
        let choice = input("Add a developer yes/no");
        let choice = choice.trim();

        if choice == "no" {
            break;
        } else if choice != "yes" {
            println!("Invalid input. Please answer with yes or no");
            continue;
        }

        let name = input("Developer's Name");
        let work_exp: u32 = input("Developer's Work Experience")
            .trim()
            .parse()
            .expect("Input is not a number");

        let dev_data = (name, work_exp);

        if dev_data.1 > best.1 {
            best = dev_data.clone();
        }

        developers.push(dev_data);
    }

    println!("The developer with the highest work experience is:");
    println!("Name: {}", best.0);
    println!("Work Experience: {}", best.1);
    println!("All Devs {:?}", developers);
}

fn input(que: &str) -> String {
    println!("{}", que);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input
}