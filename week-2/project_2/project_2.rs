fn main() {
	/*P.M. Okeke and Sons Ltd is downsizing and readjusting their product sales due to an ongoing recession. You have been
consulted to write a Rust program that calculates the sum and the average of the following sales record.
*/
let toshiba = 450_000.00;
let mac = 1_500_000.00;
let hp = 750_000.00;
let dell = 2_850_000.00;
let acer = 250_000.00;
let quantity = 2.0+1.0+3.0+3.0+1.0;
let sum = (2.0*toshiba) + (mac) + (3.0*hp) + (3.0*dell) + (acer);
let average = sum/quantity;
println!("The sum and average of the sales record are'{}' and '{}' respectively", sum, average);
// What I learn here was to just use decimals for every variable to have peace of mind in the code and less f64 compiling errors

}