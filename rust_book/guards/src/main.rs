//src: https://doc.rust-lang.org/nightly/book/patterns.html#guards

fn main() {
    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(6);

    match x {
        OptionalInt::Value(i) if i > 6 => println!("Got an int>6!"),
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(i) if i > 7 => println!("Got an int>7! will never trigger due to order!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }
}

