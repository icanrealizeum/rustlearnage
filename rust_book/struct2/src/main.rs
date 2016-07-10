fn main() {
    #[derive(Debug)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let p = Point3d { x: 3, y: 4, z: 5 };
    let mut point = Point3d { x: 0, y: 0, z: 0 };
    println!("{:?}", point);
    point = Point3d { y: 1, ..p };
    println!("{:?}", point);
    //src: https://doc.rust-lang.org/nightly/book/structs.html#update-syntax
}

