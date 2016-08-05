//https://rust-lang.github.io/book/ch04-04-slices.html

fn first_word(s: &String) -> usize {

    // Since we need to go through the String element by element, and
    // check if a value is a space, we will convert our String to an
    // array of bytes, using the `.as_bytes()` method.
    let bytes = s.as_bytes();

    // We discussed using the iter() method with for in Chapter 3.7. Here,
    // we’re adding another method: enumerate(). While iter() returns each
    // element, enumerate() modifies the result of iter(), and returns a
    // tuple instead. The first element of the tuple is the index, and the
    // second element is a reference to the element itself. This is a bit
    // nicer than calculating the index ourselves.
    //
    // Since it’s a tuple, we can use patterns, just like elsewhere in Rust.
    // So we match against the tuple with i for the index, and &byte for
    // the byte itself.
    //for (i, &byte) in bytes.iter().enumerate() {
    let heart = '❤';//lol wait, outdated doc? https://doc.rust-lang.org/std/primitive.char.html#representation
    println!("{}", heart);//yup, looks like it - rust nightly can do this.
    //let heart2 = '❤️'; //oh, wait - nevermind! I guess this looks the same but it's different!!! - aka this errors!
    for (i, chr) in s.chars().enumerate() {

        //println!("Processing {} {}",i,byte);
        println!("Processing {} {}",i,chr);
        // 32 is the value of a space in UTF-8
        //if chr == 0x20 {
        if chr == '\u{0020}' {

            // We found a space! Return this position.
            return i;
        }
    }

    // If we got here, we didn’t find a space, so this whole thing must be a
    // word. So return the length.
    s.len()
}

fn main() {
    //ok, apparently 0x20 isn't part of any unicode char seen as utf-8 as far as i tested until
    //here(not all):
    //http://www.utf8-chartable.de/unicode-utf8-table.pl?start=121856&number=1024&utf8=0x
    //about to search here next(as soon as internet is back):
    //http://www.fileformat.info/search/search.htm?q=0x20
    //ok this explains it: http://www.joelonsoftware.com/articles/Unicode.html
    //since high bits are always at least 10, there won't be any 0x20 dangling around as part of
    //the utf8 represented unicode - except 0x20 itself :D
    let s=String::from("❤️Ԡ12\u{14}3԰4Ġ❤️we❤️ll❤ok");//ok this last ❤ is one codepoint!!!
    //❤️ is two unicode code points which is why you'll see a blank after it!
    //src: https://doc.rust-lang.org/std/primitive.char.html
    let word = first_word(&s);
    // word is now totally invalid! There’s no more word here.
    println!("{}", '\u{2764}');//since this is ... good enough for the heart!
    println!("{}", '\u{fe0f}');//why is this needed? well, ok: https://www.fileformat.info/info/unicode/char/2764/index.htm variation selector
    println!("{}",word);
    println!("heart.len {}",String::from("❤️").len());//6
}
