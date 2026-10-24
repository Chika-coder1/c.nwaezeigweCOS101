/*MAKE FULL VECTORS FOR EACH COLUMN
AND PRINT THEM OUT*/

use::std::io::Write;
fn main() {

    let name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Bianca Edemoh"];
    let matric_no = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let dept = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let lvl = vec!["300","100","200","200","100"];
    
    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("Create failed");

    writeln!(file, "Name,matric_no,Department,level").expect("Failed to write headers");
    for i in 0..name.len() {

        let row = format!("{},{},{},{}\n",name[i],matric_no[i],dept[i],lvl[i] );

        file.write_all(row.as_bytes()).expect("Failed to write row");
    }
}