
mod sha512;
mod sha256;
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

#[cfg(test)]
mod tests {
    use crate::Sha512;

  #[test]
  fn sha512_parity() {
    let random_text = String::from("a very random text");
    let file = std::fs::File::open("./foo.txt").expect("error opeing file");
    //file.write(random_text.as_bytes()).expect("error writting text");
    
    let x : Sha512 = crate::Hash::from_string(&random_text);
    let y : Sha512 = crate::Hash::from_file(&file);
    assert_eq!(x.hash, y.hash);
  }
}