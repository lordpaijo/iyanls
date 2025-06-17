# iyanls
My custom build of `ls` in Rust.

(Inspired and based on: [https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e](https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e))

![](https://github.com/lordpaijo/iyanls/blob/main/Screenshot_20250617_180215.png)
## Features
- Tabled and colorful output
- File type (directory / file)
- File Permissions (Normal / Octal)
- File Size (B to OB)
- Modification Time with toggling
- Grabbing files by strings
- JSON formatting

## Dependencies
- Rust (>= 1.65.0)
- Cargo (>= 1.65.0)

## Installation
```sh
$ cargo install iyanls
```

Or use the executable files from [assets](https://github.com/lordpaijo/iyanls/releases).

## Build from source
```sh
$ git clone https://github.com/lordpaijo/iyanls.git
$ cd iyanls
$ cargo build --release
```

Make sure to add the `target/release` directory to your PATH environment variable.

## Usage
```sh
$ iyanls [path] [options]
```

See `iyanls --help` for more information.
