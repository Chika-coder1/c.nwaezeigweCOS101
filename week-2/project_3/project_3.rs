/*Ms. Akudo Ijezie has recently acquired a brand new TV set, bought for N210,000. The value of the TV depreciates by 5% per
annum. Write a Rust program to find the value of the TV after 3 years. Depreciation means the reduction of value due to the
use and age of the item.
*/
fn main() {
	let p:f32 = 210_000.0;
	let r:f32 = 5.0;
	let n:f32 = 3.0;
	let a:f32 = p * (1.0 - (r/100.0)).powf (n);
	println!("The value of the TV after {} years is {}", n, a );
}