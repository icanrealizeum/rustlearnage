use std::mem;

// This function borrows a slice(or an array)
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

//oh, this borrows a slice(but not an array)!! :)
fn analyze_slice2(slice: &&[i32]) {
    analyze_slice(slice)
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
    analyze_slice(&ys);//so it borrows an array? or is it, takes a slice? same thing?
    analyze_slice2(&&ys[1 .. 4]);
    //analyze_slice2(&&ys); // err: expected slice, found array of 500 elements

    // Out of bound indexing yields a panic
    //println!("{}", xs[5]);
}
