# bad-apple-uxn

Bad Apple!! for the [Uxn](https://100r.co/site/uxn.html) stack-machine by [Hundred Rabbits](https://100r.co/site/home.html).

[![Youtube video thumbnail](https://img.youtube.com/vi/YfCyzwASzJ4/0.jpg)](https://www.youtube.com/watch?v=YfCyzwASzJ4)

## Quick start

```console
cargo run --release -- --input assets/badapple.mp4 --output badapple.tal
uxnasm badapple.tal badapple.rom
```
or download the `badapple.rom` file from the [releases page](https://github.com/karolbelina/bad-apple-uxn/releases).
```console
uxnemu badapple.rom
```

## Specifications

- Display resolution of 42x32 pixels,
- 810 frames of video (3.75 frames per second),
- 1-bit images,
- Compressed with [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding), 1 byte per run, alternating runs with no markers,
- 63375 bytes,
- No audio

## Building

This repository contains the project needed to build the `.tal` file from scratch. If you are interested only in downloading and running the file, check out the [Running](#running) section.

Make sure you have the most recent release of [Rust](https://www.rust-lang.org/), as well as FFmpeg libraries installed on your system (check out [this guide](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building) for more information on how to get them). Run the `cargo run` command with the `--release` profile, as well as the path to the input MP4 file and paths to the output files, like so:
```console
cargo run --release -- --input assets/badapple.mp4 --output badapple.tal
```
This should compile the project and run the whole build process.

You have to assemble the ROM from the `.tal` file yourself. To do that, run
```console
uxnasm badapple.tal badapple.rom
```
to assemble the ROM with [Uxnasm](https://git.sr.ht/~rabbits/uxn/tree/main/item/src/uxnasm.c).

## Running

Check out the [releases page](https://github.com/karolbelina/bad-apple-uxn/releases) for prebuilt Uxn ROM files.

Before running, make sure you have the [Uxn toolchain](https://git.sr.ht/~rabbits/uxn) installed. To run the ROM, run
```console
uxnemu badapple.rom
```
This should open a window. When you're ready, press any button on your keyboard to start the video.

## Tweaking

You can tweak various constants in the `( Tweakables )` section of the final `.tal` file before the assembly.
- `DEFAULT-SCALE` &mdash; defines the default width and height of the "video pixel" on screens with unconstrained size.
- `FRAME-TIME` &mdash; number of [Varvara](https://wiki.xxiivv.com/site/varvara.html) screen frames per each video frame. `#0001` means a 60 FPS video (video frame every screen frame), `#0010` means a _60 / 16 =_ 3.75 FPS video (video frame every 16 screen frames).

## License

This software is licensed under the MIT license.

See the [LICENSE](LICENSE) file for more details.
