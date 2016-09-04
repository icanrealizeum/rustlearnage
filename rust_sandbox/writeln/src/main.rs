//src https://gchp.ie/2016/08/09/rust-compiler-walkthrough-introduction/?utm_content=buffer12c05&utm_medium=social&utm_source=twitter.com&utm_campaign=buffer
use std::io::{Write, stdout, stderr};

fn main() {
    let mut out = stdout();
    let mut err = stderr();
    writeln!(&mut out, "Hello world1");
    writeln!(&mut err, "Hello world2");
}
