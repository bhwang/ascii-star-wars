# ASCII-STAR-WARS

This is a version of Simon Jansen's [Asciimation of Star Wars: Episode IV – A New Hope](http://www.asciimation.co.nz/) that runs in a Terminal. The original is one of my favorite things from my earliest days on the Internet. I remember finding it shortly after watching *The Phantom Menace* in theaters and being amazed at what was possible, which led to learning more about how to make webpages, programming (with embedded Java applets in CGI-bins!), and so on. Not sure how well it holds up for anyone else in today's world of high-quality streaming video, but for me, it still conjures up a little of that sense of awe. Thanks, Simon.

I've added a a couple of small "quality of life" improvements, like being able to adjust the framerate and the starting frame. See the section below on [optional flags](#optional-flags) for details.

The ASCIImation runs at 15 frames per second (fps) by default. At this framerate, it runs around 18 minutes in total. (There are around 16000 frames.)

It should look fine for any monospace font, but the frames were designed for **Courier**, which is recommended for the ideal viewing experience.

## How to run

If you have `cargo` for Rust installed, all you have to do is enter

> cargo run

in whatever directory you've copied this repository to. You can exit from it like any ordinary terminal program (e.g. "CTRL+C").

## Optional flags

There are three optional flags you can set:

* `-c` toggles a frame counter (default: disabled)
* `-f <framerate>` sets the framerate in fps (default: 15)
* `-o <offset>` starts the ASCIImation at the `<offset>`th frame (default: 0).

For example, if you wish to run the ASCIImation with the frame counter, at 30 fps, from the 3100th frame (Luke's first apperance), just input 

> cargo run -- -c -f 30 -o 3100