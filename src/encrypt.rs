use rand::{self, Rng};
use std::cmp::min;
use std::mem::swap;

#[derive(Debug)]
pub struct KeyPair {
    pub_key_b: Vec<usize>,
    w: Vec<usize>,
    q: usize,
    r: usize,
}

fn generate_superincreasing_sequence(n: usize, rng: &mut impl rand::Rng) -> (Vec<usize>, usize) {
    let mut sequence = Vec::with_capacity(n);
    let mut sum = 0usize;

    for _ in 0..n {
        let next = sum + (*rng).random_range(1..10);
        sequence.push(next);
        sum += next;
    }

    (sequence, sum)
}

fn binary_gcd(mut n1: usize, mut n2: usize) -> usize {
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

fn generate_coprime(n1: usize) -> usize {
    let mut i = 588;
    loop {
        if binary_gcd(n1, i) == 1 {
            return i;
        }
        i += 1;
    }
}

pub fn generate_key_pair(block_size: usize) -> KeyPair {
    let mut rng = rand::rng();

    let (w_seq, w_sum) = generate_superincreasing_sequence(block_size, &mut rng);
    let q = w_sum + rng.random_range(1..=100);
    let r = generate_coprime(q);
    let b = w_seq.iter().map(|wi| (wi * r) % q).collect::<Vec<usize>>();

    KeyPair {
        pub_key_b: b,
        w: w_seq,
        q,
        r,
    }
}
