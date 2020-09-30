use clap::{App, Arg};
use core::time;
use std::{
    fs,
    io::{self, Write},
    thread,
};

fn main() {
    let matches = App::new("ascii-star-wars")
        .version("0.1.0")
        .author("bhwang <github.com/bhwang>")
        .about("Play Simon Jansen's Star Wars ASCIImation in the Terminal.")
        .arg(
            Arg::with_name("framerate_in_fps")
                .value_name("fps")
                .help("Specify framerate in frames per second (fps). Default is 15 fps.")
                .short("f")
                .long("framerate"),
        )
        .arg(
            Arg::with_name("frame_offset")
                .value_name("number_of_frames")
                .help("Start the ASCIImation, skipping the inputted number of frames.")
                .short("o")
                .long("offset"),
        )
        .arg(
            Arg::with_name("include_frame_counter")
                .help("Display a frame counter below the frame while playing. (Not enabled by default.)")
                .takes_value(false)
                .short("c")
                .long("counter"),
        )
        .get_matches();

    // Parse numeric arguments
    let mut fps = 15;
    if let Some(s) = matches.value_of("framerate_in_fps") {
        match s.parse::<u64>() {
            Err(_e) => println!("Can't read framerate, setting fps = 15."),
            Ok(f) => fps = f,
        }
    }

    let mut offset = 0;
    if let Some(t) = matches.value_of("frame_offset") {
        match t.parse::<u64>() {
            Err(_e) => println!("Can't read frame offset. Starting from the beginning."),
            Ok(o) => offset = o,
        }
    }

    parse_file(fps, offset, matches.is_present("include_frame_counter"));
}

#[derive(Debug)]
struct Frame<'a> {
    frame_length: u64,
    frame: [&'a str; 13],
}

fn parse_file(framerate_in_fps: u64, frame_offset: u64, display_frame_counter: bool) {
    let filepath = "src/asciimation.txt";
    let raw_input = fs::read_to_string(filepath).expect("Should have been able to read file");
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

    play_asciimation_with_offset(
        frames,
        framerate_in_fps,
        frame_offset,
        display_frame_counter,
    );
}

/// Plays the ASCIImation at the given framerate (in fps), skipping the first `offset` frames
fn play_asciimation_with_offset(
    frames: Vec<Frame>,
    framerate_in_fps: u64,
    frame_offset: u64,
    display_frame_counter: bool,
) {
    let mut frame_counter = 0;

    let mut skipped_frames = frame_offset;
    let mut frames_to_play = Vec::new();
    for f in frames {
        if skipped_frames == 0 {
            frames_to_play.push(f);
        } else {
            let duration = f.frame_length;
            if skipped_frames >= duration {
                skipped_frames -= duration;
            } else {
                frames_to_play.push(Frame {
                    frame_length: duration - skipped_frames,
                    frame: f.frame,
                });
            }
        }
    }

    frame_counter += frame_offset;

    clear_screen();
    for current_frame in frames_to_play {
        let duration = current_frame.frame_length;

        for line in current_frame.frame {
            println!("{}", line);
        }

        frame_counter += duration;
        if display_frame_counter {
            println!("");
            println!("{}/15973", frame_counter);
        }

        io::stdout().flush().unwrap();
        clear_screen();

        let frame_duration_multiplier = 1000 / framerate_in_fps;
        thread::sleep(time::Duration::from_millis(
            duration * frame_duration_multiplier,
        ));
    }
}

// Clears screen and puts cursor at the first row and column.
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
