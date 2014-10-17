use std::iter::Unfold;

fn make_seq(start: int) -> Unfold<int, int> {
  Unfold::new(start,
      |x| if *x % 2 ==0 {
            *x = *x / 2;
            Some(*x)
          } else {
            *x = 3 * *x + 1;
            Some(*x)
          })
}

fn seq_len(start: int) -> uint {
    let a = make_seq(start);
    a.take_while(|x| *x != 1).count(|_| true) + 2
}


fn main() {
    let a = range(1, 1000000).max_by(|x| seq_len(*x));
    println!("{}", a.unwrap())

}
