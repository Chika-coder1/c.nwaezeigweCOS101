struct Laptops {
    brand:String,
    quantity:u32,
    price:u32
}

impl Laptops {
    fn total(&self)->u32 {
        self.price*3
    }
}
fn main() {
    let lap1 = Laptops {
        brand:String::from("HP"),
        quantity:10,
        price:650_000
    };
    let lap2 = Laptops {
        brand:String::from("IBM"),
        quantity:6,
        price:755_000
    };
    let lap3 = Laptops {
        brand:String::from("Toshiba"),
        quantity:10,
        price:550_000
    };
    let lap4 = Laptops {
        brand:String::from("Dell"),
        quantity:4,
        price:850_000
    };

    let total = lap1.total() + lap2.total() + lap3.total() + lap4.total();

    println!("The total amount paid by the customer that bought 3 laptops from each brand is {}",total);
}