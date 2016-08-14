// src: https://rustbyexample.com/custom_types/structs.html
// ^ I keep forgetting these, but it's obvious where the code is from!

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// Structs can be reused as fields of another struct
// and order doesn't matter!(aka forward declaration not needed)
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// A struct with two fields
#[derive(Clone,Copy)]
struct Point {
    x: f32,
    y: f32,
}


fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("area={}", rect_area(&_rectangle));
    let r=Rectangle{
        p1: Point{x:15_f32,y:10_f32},
        p2: Point{x:50_f32,y:27.0}};
    println!("area={}", rect_area(&r));
    assert_eq!(rect_area(&r), 595.0);

    let r1=Rectangle{
        p1: Point{x:48_f32,y:9_f32},
        p2: Point{x:14_f32,y:27.0}};
    println!("area={}", rect_area(&r1));
    assert_eq!(rect_area(&r1), 612.0);
    let r1_2=Rectangle{
        p1: Point{x:14_f32,y:9_f32},
        p2: Point{x:48_f32,y:27.0}};
    assert_eq!(rect_area(&r1_2), rect_area(&r1));

    let r2=square(Point{x:14.0,y:9.0}, 20.0);
    assert_eq!(rect_area(&r2), 400.0);

    let r3=square(Point{x:-1.0,y:10.0}, 20.0);
    assert_eq!(rect_area(&r3), 400.0);
}

fn rect_area(r: &Rectangle) -> f32
{
  let Rectangle{p1: Point{x,y}, p2: Point{x:x2,y:y2}} = *r;

  //src: http://www.mathopenref.com/coordrectareaperim.html
  let (width, height)= ( (y-y2).abs(), (x-x2).abs());

  width*height
}

fn square(p: Point, f: f32) -> Rectangle
{
    Rectangle{p1: p, p2: Point{x:p.x+f, y:p.y+f}}
}

