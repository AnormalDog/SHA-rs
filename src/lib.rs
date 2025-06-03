
mod sha512;
mod basic_functions;

use std::fs;

pub struct Sha512 {
  hash : [u64; 8]
}

impl Sha512 {
  pub fn print(&self) {
    for n in self.hash {
      print!("{:0>8x}", n);
    }
    println!();
  }
} // impl Sha512

/*
pub struct Sha256 {
  hash : [u32; 8]
}
*/

pub trait Hash {
  fn from_file(file : & fs::File) -> Self;
  fn from_string(string : & str) -> Self;
}

impl Hash for Sha512 {
  fn from_file(file : & fs::File) -> Self {
      return sha512::hash_from_file(file);
  }
  fn from_string(string : & str) -> Self {
      return sha512::hash_from_string(string);
  }
}