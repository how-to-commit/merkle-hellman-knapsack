mod encrypt;
mod math;
mod utils;

fn main() {
    let key_pair = encrypt::generate_key_pair(8);
    let plaintext = [0b0110_0001];
    let ciphertext = key_pair.public_key.encrypt(&plaintext);
    assert_eq!(key_pair.private_key.decrypt(ciphertext), plaintext);
}
