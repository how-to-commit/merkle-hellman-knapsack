use crate::utils::{generate_coprime, generate_superincreasing_sequence, modinv, mulmod};
use rand::{self, Rng};

pub struct PublicKey {
    key: Vec<u64>,
}

pub struct PrivateKey {
    w: Vec<u64>,
    q: u64,
    r: u64,
}

impl PublicKey {
    pub fn derive(k: &PrivateKey) -> PublicKey {
        PublicKey {
            key: k
                .w
                .iter()
                .map(|wi| mulmod(*wi, k.r, k.q))
                // .map(|wi| ((wi % k.q) * (k.r % k.q)) % k.q)
                .collect::<Vec<u64>>(),
        }
    }

    pub fn encrypt(&self, cleartext: &[u8]) -> u64 {
        let mut ci = 0;
        let mut total = 0u64;

        for bi in self.key.iter().rev() {
            let (ci_idx, ci_bit) = (ci / 8, ci % 8);
            ci += 1;

            if (cleartext[ci_idx] >> ci_bit) & 1 == 1 {
                total += bi;
            }
        }

        total
    }
}

impl PrivateKey {
    pub fn generate(block_size: usize) -> PrivateKey {
        let mut rng = rand::rng();
        let (w, w_sum) = generate_superincreasing_sequence(block_size, &mut rng);
        let q = w_sum + rng.random_range(1..=10000);
        let r = generate_coprime(q, &mut rng);
        PrivateKey { w, q, r }
    }

    pub fn decrypt(&self, ciphertext: u64) -> Vec<u8> {
        println!("decrypting...");

        let r_inv = modinv(self.r, self.q);
        let c_prime = (ciphertext * r_inv) % self.q;

        println!("found r': {r_inv}");
        println!("found c': {c_prime}");

        let mut remaining = c_prime;
        let mut message = vec![0u8; self.w.len().div_ceil(8)];

        for (i, wi) in self.w.iter().enumerate().rev() {
            if *wi <= remaining {
                println!("found w{i}: {wi}");
                let (i_idx, i_bit) = (i / 8, i % 8);
                message[i_idx] |= 1 << (7 - i_bit);
                remaining -= wi;
            }
        }

        message
    }
}

pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

pub fn generate_key_pair(block_size: usize) -> KeyPair {
    let k = PrivateKey::generate(block_size);
    let p = PublicKey::derive(&k);

    println!("Generated key pair:");
    println!("  Private key: w={:?}, q={}, r={}", k.w, k.q, k.r);
    println!("  Public key: {:?}", p.key);

    KeyPair {
        public_key: p,
        private_key: k,
    }
}
