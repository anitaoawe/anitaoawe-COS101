use std::fs::OpenOptions;
use std::io::Write;

fn main() {

    let mut file = OpenOptions::new().append(true).open("data.txt").expect(
        "cannot open file because you pay 100k school fees");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("This is the appendage to the document."
        .as_bytes()).expect("write failed because you are paying 100k school fees");
    println!("file append success");


}