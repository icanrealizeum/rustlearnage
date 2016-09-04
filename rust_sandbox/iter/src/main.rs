#![feature(inclusive_range_syntax)]
//src: https://github.com/rustomax/rust-iterators#digging-deeper

fn main() {
	for j in 1...3 {
		for i in (1..11).map(|x| x +j) {
			print!("{} ", i);
		}
        println!("");
	}
}
