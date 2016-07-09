fn get<'a>(a: bool)->&'a str {
  if a { "a" } else {"b"}
}

fn get2(a: bool)->&'static str {
  if a { "a" } else {"b"}
}

/*fn get2(a: bool)->&str { //won't work! - missing lifetime specifier
  if a { "a" } else {"b"}
}*/

fn main() {
    println!("Hello, world! {} {}", get(false), get2(true));
}

