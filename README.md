# bad-apple-uxn

Bad Apple!! for the [Uxn](https://wiki.xxiivv.com/site/uxn.html) stack-machine by [Hundred Rabbits](https://github.com/hundredrabbits).

[![Youtube video thumbnail](https://img.youtube.com/vi/YfCyzwASzJ4/0.jpg)](https://www.youtube.com/watch?v=YfCyzwASzJ4)

## Quick start

```console
cargo run -- --input assets/badapple.mp4 --output badapple.tal --rom badapple.rom
```
or download the `badapple.rom` file from the [releases page](https://github.com/karolbelina/bad-apple-uxn/releases).
```console
uxnemu badapple.rom
```

## Specifications

- Display resolution of 42x32 pixels,
- 810 frames of video (3.75 frames per second),
- 1-bit images,
- Compressed with [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding),
- 63375 bytes,
- No audio

## Building

This repository contains the project needed to build the `.tal` file from scratch. If you are interested only in downloading and running the file, check out the [Running](Running) section.

Make sure you have the most recent release of [Rust](https://www.rust-lang.org/), as well as FFmpeg libraries installed on your system (check out [this guide](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building) for more information on how to get them). Run the `cargo run` command with the path to the input MP4 file, as well as paths to the output files, like so:
```console
cargo run -- --input assets/badapple.mp4 --output badapple.tal --rom badapple.rom
```
This should compile the project and run the whole build process. Please note that it may take quite a long time.

You can omit the `--rom` flag if you want to assemble the ROM from the `.tal` file yourself. To do that, run
```console
uxnasm badapple.tal badapple.rom
```
to assemble the ROM with [Uxnasm](https://git.sr.ht/~rabbits/uxn/tree/master/item/src/uxnasm.c).

## Running

Check out the [releases page](https://github.com/karolbelina/bad-apple-uxn/releases) for prebuilt Uxn ROM files.

Before running, make sure you have the [Uxn toolchain](https://git.sr.ht/~rabbits/uxn) installed. To run the ROM, run
```console
uxnemu badapple.rom
```
This should open a window. When you're ready, press any button on your keyboard to start the video.

## License

This software is licensed under the MIT license.

See the [LICENSE](LICENSE) file for more details.
