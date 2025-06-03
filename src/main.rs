use std::fs;

use hash_rs::sha512;

fn main() {
  let x = fs::File::open("/home/anormaldog/example.zip").expect("error opening file");
  sha512::hash_from_file(&x);
}