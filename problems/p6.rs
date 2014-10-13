use std::iter::range_inclusive;

fn main() {
    let b = range_inclusive(1, 100).fold(0, |a: int, b: int| a + b);
    let c = range_inclusive(1, 100).fold(0, |a: int , b: int| a + b * b);
    println!("{}", b*b - c);
}
