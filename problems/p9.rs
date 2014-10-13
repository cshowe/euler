use std::iter::range_inclusive;
use std::iter::Repeat;

fn main() {
    let a = range_inclusive(1, 1000);
    let ab = a.flat_map(|x: int| Repeat::new(x).zip(range_inclusive(x, 1000)));
    let abc = ab.map(|(a, b)| (a, b, 1000 - a - b));
    let triple = abc.filter(|&(a, b, c)| a * a + b * b == c * c);
    let mut product = triple.map(|(a, b, c)| a * b * c);
    
    for x in product {
      println!("{}", x);
    }
}
