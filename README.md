# iyanls
My custom build of `ls` in Rust.

(Inspired and based on: [https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e](https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e))

![](https://github.com/lordpaijo/iyanls/blob/main/Screenshot_20250616_204013.png)
## Features
- A tabled and colorful output
- File type (directory / file)
- File Permissions
- File Size
- Modification Time
- Grabbing files by strings
- JSON formatting

## Dependencies
- Rust (>= 1.65.0)
- Cargo (>= 1.65.0)

## Installation
```
cargo install iyanls
```

Or use the executable files from [assets](https://github.com/lordpaijo/iyanls/releases)

## Usage
```
iyanls [path] [options]
```
Options:
- `-h` `--help`: Print help page
- `-V` `--version`: Print version
- `-g` `--grab`: Grab files that match a string
- `-j` `--json`: Output in JSON format
- `--json-export`: Export JSON data as a file
