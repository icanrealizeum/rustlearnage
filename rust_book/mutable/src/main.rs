
// src: https://doc.rust-lang.org/nightly/book/structs.html

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    //let mut a=Point { x:1, y:2};
    let point: &mut Point = &mut Point { x: 0, y: 0 }; //ugleh - immutable point, mutable struct
    //let mut point = Point { x: 0, y: 0 }; //mutable point and struct - no thanks!
    //let point: &Point = &Point { x: 0, y: 0 }; //immutable point and struct

    point.x = 5;
    //point = &mut a; //not allowed - GOOD!

    println!("The point is at ({}, {})", point.x, point.y);
}

