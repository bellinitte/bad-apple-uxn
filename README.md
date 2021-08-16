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
- Compressed with [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding), 1 byte per run, alternating runs with no markers,
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

## Tweaking

You can tweak various constants in the `( Tweakables )` section of the resulting `.tal` file before the assembly.
- `X-OFFSET` and `Y-OFFSET` &mdash; define the video offset in screen pixels from the top-left corner of the screen, can be used for centering,
- `FRAGMENT-WIDTH` and `FRAGMENT-HEIGHT` &mdash; define the size of the "video pixel", can be used for scaling the video to various screen sizes,
- `VIDEO-WIDTH` and `VIDEO-HEIGHT` &mdash; self-explanatory, can be used for _making visual glitches_,
- `FRAME-TIME` &mdash; number of [Varvara](https://wiki.xxiivv.com/site/varvara.html) screen frames per each video frame. `#0001` means a 60 FPS video (video frame every screen frame), `#0010` means a _60 / 16 =_ 3.75 FPS video (video frame every 16 screen frames),
- `STOP-TIME` &mdash; frame index after which the rendering terminates.

## License

This software is licensed under the MIT license.

See the [LICENSE](LICENSE) file for more details.
