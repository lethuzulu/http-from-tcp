use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::{fs::File, io::Read, str::from_utf8};

fn main() {
    let  file = File::open("message.txt").unwrap();
    let receiver = get_lines_channel(file);

    loop {
        match receiver.recv() {
            Ok(v) => println!("{}", v),
            Err(_) => break,
        }
    }
}

fn get_lines_channel(mut file: File) -> Receiver<String> {
    let (sender, receiver) = channel::<String>();

    let _handle = thread::spawn(move || {
        let mut buf = [0u8; 8];

        let mut current_line = String::new();
        loop {
            let size = file.read(&mut buf).unwrap();
            if size == 0 {
                break;
            }

            let chunk = from_utf8(&buf[..size]).unwrap();

            let parts: Vec<&str> = chunk.split('\n').collect();

            for part in &parts[..parts.len() - 1] {
                current_line.push_str(part);
                let _ = sender.send(current_line.clone()).unwrap();
                current_line.clear();
            }

            current_line.push_str(parts.last().unwrap());
        }

        if !current_line.is_empty() {
            let _ = sender.send(current_line).unwrap();
        }
    });
    receiver
}
