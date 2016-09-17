#![feature(inclusive_range_syntax)]
//src: https://github.com/rustomax/rust-iterators#digging-deeper

fn main() {
    for j in 1...3 {
        for i in (1..11).map(|x| x +j) {
            print!("{} ", i);
        }
        println!("");
    }

    let v = vec![3, 5, 0, -2, 3, 4, 1];
//    let v:Vec<i32> = vec![];
    let max = v.iter().max();
    let min = v.iter().min();

    println!("max = {:?}, min = {:?}", max, min);

    // output: max = Some(5), min = Some(-2)
}
