//src: https://rustbyexample.com/primitives/tuples.html

use std::fmt;

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

//this moves
fn transpose(m: Matrix) -> Matrix {
    // `let` can be used to bind the members of a tuple to variables
    let m=Matrix(m.0,m.2,m.1,m.3);

    m
}

//this borrows
fn transpose2(m: &Matrix) -> Matrix {
    // `let` can be used to bind the members of a tuple to variables
    let m=Matrix(m.0,m.2,m.1,m.3);

    m
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
	  // Write strictly the first element into the supplied output
	  // stream: `f`. Returns `fmt::Result` which indicates whether the
	  // operation succeeded or failed. Note that `write!` uses syntax which
	  // is very similar to `println!`.
	  try!(write!(f, "( {} {} )\n", self.0, self.1));
	  write!(f, "( {} {} )", self.2, self.3)//no ";" so to return a fmt::Result value
  }
}

fn main() {
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
	println!("Transpose2:\n{}", transpose2(&matrix));//moves!
    println!("Matrix:\n{}", matrix);
	println!("Transpose:\n{}", transpose(matrix));//moves!
    //println!("Matrix:\n{}", matrix);//can't - moved!

}
