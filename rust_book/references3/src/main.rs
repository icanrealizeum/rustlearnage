fn main() {
    let seven = 7;
    let eight = 8;

    let mut x = 5;
    let mut z = 6;
    {
        let mut y = &mut x;
        // y=&mut seven;
        *y = seven;

        // {
        y = &mut z;
        // y=&mut eight;
        *y = eight;
        // }
        println!("{:?}", y);
    }
    println!("{:?}", x);
    println!("{:?}", z);
}

