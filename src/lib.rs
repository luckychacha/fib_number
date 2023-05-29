use std::mem::replace;

use num_bigint::BigUint;
use num_traits::{One, Zero};
// F0 = 0
// F1 = 1
// F2 = F0 + F1
// F(n+2) = Fn + F(n+1)
pub fn fib(n: u64) -> BigUint {
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

    // let mut dp: (BigUint, BigUint) = (Zero::zero(), One::one());
    // for _ in 2..=n {
    //     dp = (dp.1.clone(), dp.0.clone() + dp.1.clone())
    // }
    // dp.1
}

/*
| b  a |   | 1  1 |   | a+b  b |
| 0  0 | * | 1  0 | = | 0    0 |

| F(1)  F(0) |   | 1  1 |   | F(2)[F(0)+F(1)]  F(1) |
| 0      0   | * | 1  0 | = | 0                 0   |
 */
pub fn fib_matrix(n: u64) -> BigUint {
    if n < 2 {
        return n.into();
    }

    let mut base_matrix = [[One::one(), One::one()], [One::one(), Zero::zero()]];
    let mut result_matrix = [[One::one(), Zero::zero()], [Zero::zero(), Zero::zero()]];

    let mut k = n - 1;
    while k > 0 {
        if k % 2 == 1 {
            result_matrix = matrix_multiply(&result_matrix, &base_matrix);
        }

        base_matrix = matrix_multiply(&base_matrix, &base_matrix);
        k /= 2;
    }
    result_matrix[0][0].clone()
}

fn matrix_multiply(a: &[[BigUint; 2]; 2], b: &[[BigUint; 2]; 2]) -> [[BigUint; 2]; 2] {
    let mut res: [[BigUint; 2]; 2] = [
        [0_usize.into(), 0_usize.into()],
        [0_usize.into(), 0_usize.into()],
    ];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                res[i][j] += a[i][k].clone() * b[k][j].clone();
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fib_should_work() {
        assert_eq!(fib(0), 0.try_into().unwrap());
        assert_eq!(fib(1), 1.try_into().unwrap());
        assert_eq!(fib(2), 1.try_into().unwrap());
        assert_eq!(fib(3), 2.try_into().unwrap());
        assert_eq!(fib(4), 3.try_into().unwrap());
    }

    #[test]
    fn fib_matrix_should_work() {
        assert_eq!(fib_matrix(0), 0.try_into().unwrap());
        assert_eq!(fib_matrix(1), 1.try_into().unwrap());
        assert_eq!(fib_matrix(2), 1.try_into().unwrap());
        assert_eq!(fib_matrix(3), 2.try_into().unwrap());
        assert_eq!(fib_matrix(4), 3.try_into().unwrap());
        assert_eq!(fib_matrix(1_000), fib(1_000));
    }
}
