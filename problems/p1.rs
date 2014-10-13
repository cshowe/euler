extern crate debug;

fn main() {
    let a = range(0, 1000).filter(|x: &int| (x % 3 == 0) || (x % 5 ==0 )).fold(0, |x, y| x + y);
    println!("{:?}", a);
}
