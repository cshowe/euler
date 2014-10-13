extern crate debug;
use std::iter::Unfold;

fn main() {
    let a = Unfold::new((1, 2), |z| {let (x, y) = *z; *z = (y, x + y); Some(x)});
    let b = a.take_while(|x: &int| *x <= 4000000).filter(|x| x % 2 == 0).fold(0, |x, y| x + y);
    println!("{:?}", b)
}
