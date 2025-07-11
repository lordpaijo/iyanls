# iyanls

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/iyanls)](https://crates.io/crates/iyanls)
[![GitHub stars](https://img.shields.io/github/stars/lordpaijo/iyanls)](https://github.com/lordpaijo/iyanls/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/lordpaijo/iyanls)](https://github.com/lordpaijo/iyanls/issues)
[![GitHub forks](https://img.shields.io/github/forks/lordpaijo/iyanls)](https://github.com/lordpaijo/iyanls/network/members)
[![GitHub commits](https://img.shields.io/github/commit-activity/m/lordpaijo/iyanls)](https://github.com/lordpaijo/iyanls/commits/main)
[![MIT License](https://img.shields.io/github/license/lordpaijo/iyanls)](https://github.com/lordpaijo/iyanls/blob/main/LICENSE)

Iyanls is an open-source, powerful and flexible alternative tool to GNU Core Utilities `ls`. Written on top of the Rust programming language, it offers a wide range of features and robust power for a searching tool.

![](https://github.com/lordpaijo/iyanls/blob/main/ss00.png)
![](https://github.com/lordpaijo/iyanls/blob/main/ss01.png)
## Features
- Tabled and colorful output
- File type (directory/file)
- File Permissions (Normal / Octal / Users)
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

Make sure to add `.cargo/bin` to your PATH environment variable.

## Build from source
```sh
$ git clone https://github.com/lordpaijo/iyanls.git
$ cd iyanls
$ cargo build --release
```

Make sure to add the `target/release` directory to your PATH environment variable.

## Usage
```
$ iyanls | ils [path] [options]
```

Examples:
- List files in the current directory:
  ```
  $ iyanls | ils
  ```
- List files in a specific directory:
  ```
  $ iyanls | ils /path/to/directory
  ```
- List files with deep processing:
  ```
  $ iyanls | ils --deep /path/to/directory
  ```
- List files with JSON formatting:
  ```
  $ iyanls | ils --json /path/to/directory
  ```
- List files that match a string:
  ```
  $ iyanls | ils --grab string /path/to/directory
  ```

See `iyanls -h` for more information, or read [the documentation](https://github.com/lordpaijo/iyanls/blob/main/docs.md).

---

Inspiration: [https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e](https://youtu.be/5UA9UWWAagc?si=nceQeo-33Cpjpb-e)
