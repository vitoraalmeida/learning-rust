use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("poem.txt").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break
        }
        println!("{} ({} bytes long)", line, len);
        line.truncate(0);
    }

    /* could be:
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
    */

}
