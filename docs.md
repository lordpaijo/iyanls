# iyanls Documentation
## Introduction
Iyanls is an open-source, powerful and flexible alternative tool to GNU Core Utilities `ls`. Written on top of the Rust programming language, it offers a wide range of features and robust power for a searching tool.

## Installation
To install iyanls, you can use the following command:

```bash
$ cargo install iyanls
```

This will install the iyanls package which has two binariesL `iyanls` and `ils`. Both are the same, and the two work as an alias to one another.

## Usage
Once installed, you can use iyanls by running the following command:

```bash
$ iyanls [options] [path]
```

## Features
Iyanls has many in-built features in a form of CLI arguments. These features are designed to enhance the user experience and provide more control over the output.

### Help
The help page provides information about the available options and their usage. To get help on the available options, you can use the following command:

```bash
$ iyanls [ -h | --help ]
```

---

### Version
Version page helps you know what version of iyanls you are using. To get the version of iyanls, you can use the following command:

```bash
$ iyanls [ -V | --version ]
```

---

### Deep Processing
Deep processing allows iyanls to process files and directories recursively and show their detailed size. To enable deep processing, you can use the following command:

```bash
$ iyanls [ -d | --deep ] [path]
```

Warning: Deep processing can be resource-intensive and may take a long time to complete when used in a large directory structure like the root and the home directory.

---

### Grab
Grab is a feature mimicing the `grep` command. It shows you files and directories that match a specific string or pattern. To use grab, you can use the following command:

```bash
$ iyanls [ -g | --grab ] [pattern] [path]
```

---

### Re-Grab
Re-Grab is the opposite of Grab. Instead of showing the files and directories that match the pattern, it shows the files and directories that do not match the pattern. To use re-grab, you can use the following command:

```bash
$ iyanls [ -r | --re-grab ] [pattern] [path]
```

---

### JSON Formatting
JSON Formatting allows iyanls to output the results in a JSON format. To enable json formatting, you can use the following command:

```bash
$ iyanls [ -j | --json ] [path]
```

---

### JSON Exporting
JSON Exporting allows iyanls to export the results in a JSON format to a file. To enable json exporting, you can use the following command:

```bash
$ iyanls [ --jsx | --json-export ] [path]
```

---

### Line Numbers Toggling
You can toggle line numbers off using the following command:

```bash
$ iyanls [ -n | --no-line-numbers ] [path]
```

By default, line numbers are always enabled.

---

### Permissions Metadata Format
There are three available types of permissions format:

1. **Symbolic** (default)
2. **Octal**
3. **Owner / Users**

The default format is Symbolic. To use the other two, you can use the following commands:

Octal Permissions:
```bash
$ iyanls [ -o | --octal-perms ] [path]
```

User Permissions:
```bash
$ iyanls [ -u | --owner-perms ] [path]
```

---

### Print Current Working Directory Metadata
To print your current working directory metadata, you can use the following command:

```bash
$ iyanls [ -a | --show-cwd ] [path]
```

---

### Include Files or Directories
If you want to read other files or directories metadata outside or inside your current working directory, you can use the following command:

```bash
$ iyanls [ -i | --include ] [pattern] [path]
```

---

### Exclude Files or Directories
If you want to exclude files or directories metadata outside or inside your current working directory, you can use the following command:

```bash
$ iyanls [ -e | --exclude ] [pattern] [path]
```

This works the same way as re-grab. It's just more formal.

---

### Time Formats
You can specify your time format (utc, local, unix, iso8601, rfc3339, utf, custom), by using the following command:

```bash
$ iyanls [ -t | --time-format ] [format] [path]
```

---

### Custom Time Format
You can specify your custom time format by using the following command:

```bash
$ iyanls --custom-time-format [format] [path]
```

---

### Timezone
You can specify your timezone by using the following command:

```bash
$ iyanls --timezone [timezone] [path]
```

---

### Toggling Clock
To toggle the clock on or off, you can use the following command:

```bash
$ iyanls --toggle-clock [path]
```

---

## Sortings
There are many file sortings in Iyanls, and you can choose which one you want to use. (default set to none)

### Sorting by Modification Time
You can choose between sorting by newest or oldest using these commands:

Newest:
```bash
$ iyanls [ -U | up-to-date ] [path]
```

Oldest:
```bash
$ iyanls [ -D | down-to-date ] [path]
```

---

### Sorting by Size
You can choose between sorting by the largest or the smallest in size using these commands:

largest:
```bash
$ iyanls [ -X | --largest-size ] [path]
```

smallest:
```bash
$ iyanls [ -S | --smallest-size ] [path]
```

---

### Sorting by Alphabetical Order
You can choose between sorting by alphabetical order or reverse alphabetical order using these commands:

Alphabetical:
```bash
$ iyanls [ -A | --alphabetical-order ] [path]
```

Reverse Alphabetical:
```bash
$ iyanls [ -B | --alphabetical-reverse ] [path]
```

---

### Sorting by Directories
You can choose between sorting by directories first or last using these commands:

Directories First:
```bash
$ iyanls [ -C | --dir-first ] [path]
```

Directories Last:
```bash
$ iyanls [ -L | --dir-last ] [path]
```
