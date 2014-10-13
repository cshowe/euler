pub mod prime {
    use std::iter::Unfold;
    use std::vec::Vec;

    fn is_prime(primelist: &Vec<int>, x: int) -> bool {
        let a = primelist.iter();
        !a.take_while(|y: &&int| (**y)*(**y) <= x).any(|y| x % *y == 0)
    }

    fn unfold_prime(primelist: &mut Vec<int>) -> Option<int> {
        let val : int = *primelist.last().unwrap();
        let mut x = val + 1;
        while !is_prime(primelist, x) {
          x = x + 1;
        }
        primelist.push(x);
        Some(val)
    }

    pub fn primes<'r>() -> Unfold<'r, int, Vec<int>> {
      Unfold::new(vec!(2), unfold_prime)
    }
}
