extern crate collections;
use std::collections::hashmap::HashMap;


fn next(x: int) -> int {
    if x % 2 ==0 {
        x /2
    } else {
        3 * x + 1
    }
}

fn seq_len(cache: &mut HashMap<int, uint>, start: int) -> uint {
    match cache.find(start) {
        Some(x) => x,
        None => {
          let x = 1 + seq_len(cache, next(start));
          cache.insert(start, x);
          x
        }
    }
}


fn main() {
    let mut cache = HashMap::new();
    cache.insert(1, 1);
    let a = range(1, 1000000).max_by(|x| seq_len(cache, *x));
    println!("{}", a.unwrap())

}
