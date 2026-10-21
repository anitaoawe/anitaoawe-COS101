use std::fs;

fn main() {

   fs::remove_file("data.txt").expect("You are paying 100k school fees thats why you cant get it");
   println!("file is removed");
}
