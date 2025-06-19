# iyanls
My custom build of `ls` in Rust.

(Inspired and based on: [https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e](https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e))

![](https://github.com/lordpaijo/iyanls/blob/main/ss00.png)

## Features
- Tabled and colorful output
- File type (directory / file / users)
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
$ iyanls | ils [path] [options]
```

Examples:
- List files in the current directory:
  ```sh
  $ iyanls | ils
  ```
- List files in a specific directory:
  ```sh
  $ iyanls | ils /path/to/directory
  ```
- List files with deep processing:
  ```sh
  $ iyanls | ils --deep /path/to/directory
  ```
- List files with JSON formatting:
  ```sh
  $ iyanls | ils --json /path/to/directory
  ```
- List files that match a string:
  ```sh
  $ iyanls | ils --grab string /path/to/directory
  ```

See `iyanls --help` for more information.
