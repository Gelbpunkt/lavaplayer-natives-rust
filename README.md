# lavaplayer-natives-rust

Attempt to port the [lavaplayer natives](https://github.com/sedmelluq/lavaplayer/tree/master/natives/connector) to pure Rust implementations to simplify the building for different platforms.

## Current progress

The project has all function signatures ported over and lacks implementations.

## Decoders

| Codec     | Rust library to use | Implemented?   |
| --------- | ------------------- | -------------- |
| AAC       | undecided           | No             |
| MP3       | rinimp3             | Yes (untested) |
| Vorbis    | undecided           | No             |
| Opus      | undecided           | No             |

## Encoders

| Codec     | Rust library to use | Implemented?                                                             |
| --------- | ------------------- | ------------------------------------------------------------------------ |
| Opus      | undecided           | Yes, for testing using opus-sys (untested and needs pure rust in future) |

## Misc

| Task       | Rust library to use | Implemented? |
| ---------- | ------------------- | ------------ |
| Statistics | undecided           | No           |
| Samplerate | undecided           | No           |
