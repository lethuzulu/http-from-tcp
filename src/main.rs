use std::{fs::File, io::Read, str::from_utf8};

fn main() {
    let mut file = File::open("message.txt").unwrap();
    let mut buf = [0u8; 8];

    let mut current_line = String::new();

    loop {
        let count = file.read(&mut buf).unwrap();
        if count == 0 {
            break;
        }
        let chunk = from_utf8(&buf[0..count]).unwrap();

        let parts = chunk.split('\n').collect::<Vec<&str>>();

        for part in &parts[0..parts.len() - 1] {
            current_line.push_str(part);
            println!("Line {}", current_line);
            current_line.clear();
        }

        current_line.push_str(parts.last().unwrap());
    }

    if !current_line.is_empty() {
        println!("Line {}", current_line)
    }
}
