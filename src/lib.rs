use std::mem::replace;

use num_bigint::BigUint;
use num_traits::{One, Zero};
// F0 = 0
// F1 = 1
// F2 = F0 + F1
// F(n+2) = Fn + F(n+1)
pub fn fib(n: usize) -> BigUint {
    if n < 2 {
        return n.into();
    }

    let mut dp_0: BigUint = Zero::zero();
    let mut dp_1: BigUint = One::one();
    for _ in 2..=n {
        let dp_2 = dp_0 + &dp_1;
        dp_0 = replace(&mut dp_1, dp_2);
    }

    dp_1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fib_should_work() {
        // assert_eq!(fib(0), 0.try_into().unwrap());
        // assert_eq!(fib(1), 1.try_into().unwrap());
        assert_eq!(fib(2), 1.try_into().unwrap());
        assert_eq!(fib(3), 2.try_into().unwrap());
    }
}
