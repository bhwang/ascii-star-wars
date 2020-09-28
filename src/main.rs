use core::time;
use std::{
    fs,
    io::{self, Write},
    thread,
};

#[derive(Debug)]
struct Frame<'a> {
    frame_length: u64,
    frame: [&'a str; 13],
}

fn main() {
    let raw_input =
        fs::read_to_string("src/asciimation.txt").expect("Should have been able to read file");
    let raw_frame_lines: Vec<&str> = raw_input.split('\n').collect();

    // We just want to parse the input text into "frames of asciimation".
    // A frame of asciimation consists of 14 lines, where the first line
    // contains the duration with which to display the remaining 13 lines.
    let mut frames = Vec::new();
    for chunk in raw_frame_lines.chunks(14) {
        let frame_length = match chunk[0].parse::<u64>() {
            Err(_e) => 0,
            Ok(n) => n,
        };

        if frame_length == 0 {
            continue;
        }

        let mut frame = [""; 13];
        for i in 1..=13 {
            frame[i - 1] = chunk[i];
        }

        frames.push(Frame {
            frame_length,
            frame,
        })
    }

    for f in frames {
        let duration = f.frame_length;

        for line in f.frame {
            println!("{}", line);
        }

        io::stdout().flush().unwrap();
        print!("\x1B[2J\x1B[1;1H"); // clear screen and reset cursor
        thread::sleep(time::Duration::from_millis(duration * 1000 / 15));
    }
}
