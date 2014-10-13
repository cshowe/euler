extern crate util;

fn main() {
    let mut b = util::prime::primes().take_while(|&x| x < 2000000);
    let c = b.fold(0, |x, y| x + y);
    println!("{}", c);
}
