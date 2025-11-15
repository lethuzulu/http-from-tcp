
use std::{fs::File, io::Read, str::from_utf8};

fn main() {

   let mut file = File::open("message.txt").unwrap();
   let mut buf = [0u8;8];

   loop {
   let count = file.read(&mut buf).unwrap();
   if count == 0 { break }
   let chunk = from_utf8(&buf[0..count]).unwrap();
   println!("read {}", chunk);
   }



}
