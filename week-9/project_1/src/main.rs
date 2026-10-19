use::std::io::Write;

fn main() {
    let header = "Lager, Stout, Non Alchoholic";
    let row1 = "\n33_Export, Legend, Maltina";
    let row2 = "\nDesperados, Turbo_King, Amstel Malta";
    let row3 = "\nGoldberg, Williams, Malta Gold";
    let row4 = "\nGulder, _, Fayrouz";
    let row5 = "\nHeineken ,_, _";
    let row6 = "\nStar, _, _, ";

    let mut file = std::fs::File::create("Nigerian_Breweries.csv").expect("Create failed");
    file.write_all(header.as_bytes()).expect("write_all failed");
    file.write_all(row1.as_bytes()).expect("write_all failed");
    file.write_all(row2.as_bytes()).expect("write_all failed");
    file.write_all(row3.as_bytes()).expect("write_all failed");
    file.write_all(row4.as_bytes()).expect("write_all failed");
    file.write_all(row5.as_bytes()).expect("write_all failed");
    file.write_all(row6.as_bytes()).expect("write_all failed");
}
