fn main() {
    let mut x = 5;
	let mut _y=10;
{
//    let &mut y: &mut u64 = &mut x;
//    println!("{}", y);
    let mut y: &mut u64 = &mut x;
    *y += 1;
    y = &mut _y;
    *y+=1;
}
println!("{} {}", x, _y);
}
