use std::fs;

use hash_rs::sha512;

fn main() {
  let x = fs::File::open("/home/anormaldog/foo.txt").expect("error opening file");
  sha512::hash_from_file(&x);
}