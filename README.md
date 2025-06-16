# iyanls
My custom build of `ls` in Rust.

(Inspired and based on: [https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e](https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e))

![](https://github.com/lordpaijo/iyanls/blob/main/Screenshot_20250616_204013.png)

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

Examples:
- List files in the current directory:
  ```sh
  $ iyanls
  ```
- List files in a specific directory:
  ```sh
  $ iyanls /path/to/directory
  ```
- List files with deep processing:
  ```sh
  $ iyanls -d /path/to/directory
  ```
- List files with JSON formatting:
  ```sh
  $ iyanls --json /path/to/directory
  ```
    - List files that match a string:
  ```sh
  $ iyanls -g string /path/to/directory
  ```

See `iyanls --help` for more information.
