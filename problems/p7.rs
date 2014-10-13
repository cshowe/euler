use std::iter::Unfold;
use std::vec::Vec;

fn is_prime(primelist: &Vec<int>, x: int) -> bool {
    let a = primelist.iter();
    !a.take_while(|y: &&int| (**y)*(**y) <= x).any(|y| x % *y == 0)
}

fn unfold_prime(primelist: &mut Vec<int>) -> Option<int> {
    let mut x = primelist.last().unwrap() + 1;
    while !is_prime(primelist, x) {
      x = x + 1;
    }
    primelist.push(x);
    Some(x)
}

fn main() {
    let mut a = Unfold::new(vec!(2), unfold_prime);
    let b = a.nth(9999);
    println!("{}", b);
}
