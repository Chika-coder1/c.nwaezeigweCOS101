use std::io::Write;
use std::io::Read;  
use std::fs::File;
fn main() {

    let mut commissioner = File::open("comm.csv").unwrap();
    let mut comm = String::new();
    commissioner.read_to_string(&mut comm).unwrap();
    let comm_order: Vec<&str> = comm.trim().lines().collect();
    println!("{:?}",  comm_order);

    /////////////

    let mut ministry = File::open("ministry.csv").unwrap();
    let mut mini = String::new();
    ministry.read_to_string(&mut mini).unwrap();
    let ministry_order: Vec<&str> = mini.trim().lines().collect();
    println!("{:?}",  ministry_order);

    /////////////////////


    let mut zone = File::open("zone.csv").unwrap();
    let mut zo = String::new();
    zone.read_to_string(&mut zo).unwrap();
    let zone_order: Vec<&str> = zo.trim().lines().collect();
    println!("{:?}",  zone_order);


    ////////////////////////////////

    let mut file = std::fs::File::create("EFCC_Merge.csv").expect("Create failed");
    writeln!(file, "Commissioner,Ministry,Zone").expect("Failed to write headers");

    let total_rows = comm_order.len()
        .min(ministry_order.len())
        .min(zone_order.len());


    for i in 0..total_rows {

        let row = format!("{},{},{}\n",comm_order[i],ministry_order[i],zone_order[i]);
        file.write_all(row.as_bytes()).expect("Failed to write row");
    }

    println!("All {} rows are merged into EFCC_Merge.csv", total_rows);

}