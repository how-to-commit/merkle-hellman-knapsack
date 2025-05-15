use std::cmp::min;
use std::mem::swap;

pub fn generate_superincreasing_sequence(n: usize, rng: &mut impl rand::Rng) -> (Vec<u64>, u64) {
    let mut sequence = Vec::with_capacity(n);
    let mut sum = 0u64;

    for _c in 0..n {
        // calculate bound before overflow
        // let limit = u64::MAX - sum - u64::try_from(n - _c).expect("u64 >= usize");
        let limit = 300; // instead of dynamic, we limit growth per time to 300

        let next = sum + (*rng).random_range(1..limit);
        sequence.push(next);
        sum += next;
    }

    (sequence, sum)
}

pub fn generate_coprime(n1: u64, rand: &mut impl rand::Rng) -> u64 {
    let lower_bound = n1 / 2;
    let upper_bound = n1 * 2;
    let mut i: u64;

    loop {
        i = rand.random_range(lower_bound..=upper_bound);
        if binary_gcd(n1, i) == u64::from(1u16) {
            return i;
        }
    }
}

pub fn binary_gcd(mut n1: u64, mut n2: u64) -> u64 {
    // Base case: gcd(n, 0) = gcd(0, n) = n
    if n2 == 0 {
        return n1;
    } else if n1 == 0 {
        return n2;
    }

    let i = n1.trailing_zeros();
    n1 >>= i;
    let j = n2.trailing_zeros();
    n2 >>= j;
    let k = min(i, j);

    loop {
        if n1 > n2 {
            swap(&mut n1, &mut n2)
        }
        n2 -= n1;

        if n2 == 0 {
            return n1 << k;
        }

        n2 >>= n2.trailing_zeros();
    }
}

// Extended Euclidean algorithm computes ax + by = gcd(a, b), where x and y are
// the coefficients of Bezout's Identity, and in the case where they are
// coprime, ax % b = 0. (the modular multiplicative inverse)
pub fn modinv(a: u64, b: u64) -> u64 {
    let (mut s, mut old_s) = (0i64, 1i64);
    let (mut r, mut old_r) = (b as i64, a as i64);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }

    ((old_s + b as i64) % b as i64) as u64
}
