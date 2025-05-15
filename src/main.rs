mod encrypt;

fn main() {
    let x = encrypt::generate_key_pair(8);
    println!("{:?}", x);
}
