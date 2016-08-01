fn main() {
    let mut s1 = String::from("hello");

    let len:i32;
    {
      (s1, len) = calculate_length(s1); // how to do this without a 'let s1' or without a let in general?
      //s1=s2;
    }

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
