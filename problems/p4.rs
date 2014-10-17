use std::iter::range_inclusive;

fn is_palindrome(x: int) -> bool {
    let a = x.to_string();
    a.as_bytes().iter().zip(a.as_bytes().iter().rev()).all(|(x, y)| x == y)
}

fn main() {
    let a = range_inclusive(100, 999).flat_map(|x: int| { range_inclusive(100, 999).map(|y: int| x * y)} );
    for x in range_inclusive(100, 999) {
        for y in range_inclusive(100, 999) {
            if is_palindrome(x * y) {
                println!("{}", x *y)
            }
        }
    }
}
