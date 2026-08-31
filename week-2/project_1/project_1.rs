fn main() {
	/* The Ibeju Local Government Chairman has received a mortgage loan of N520,000,000 from Sterling Bank for the
construction of the Lekki Free Trade Zone industrial estate. Find the compound interest for 5 years at 10% per annum,
compounded annually.
	*/
	let p:f64 = 520_000_000.0;
	let t:f64 = 5.0;
	let r:f64 = 10.0;
	let a = p * (1.0 + (r/100.0)).powf(t);
	let ci = a - p;
	println!("The compound interest for 5 years at 10% per annum is {}",ci );

}