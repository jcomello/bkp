# bkp 🦀

A lightweight, fast, and reliable Command Line Interface (CLI) built in Rust to create backups of files

## Features

- **Fast & Efficient**: Built on Rust's high-performance file I/O operations.
- **Cross-Platform**: Works seamlessly on Windows, macOS, and Linux.

## Installation

### From Source

Ensure you have the Rust toolchain installed. If not, get it at [rustup.rs](https://rustup.rs).

```bash
# Clone the repository
git clone https://github.com/jcomello/bkp
cd bkp

# Build and install locally
cargo install --path .
```

## Usage

Once installed, the `bkp` command will be available in your terminal.

### Basic Syntax

```bash
Usage: bkp [OPTIONS] <PATH> [TARGET_DIRECTORY]

Arguments:
  <PATH>              File path to be backed up
  [TARGET_DIRECTORY]  Target directory path. If target directory is not given, the backup file will be saved in the same directory as the original file

Options:
  -f, --format <FORMAT>  Changes the date format for the backup file [default: %Y%m%d%H%M]
  -h, --help             Print help
  -V, --version          Print version
```

### Examples

**1. Back up a file to the current directory:**
```bash
bkp document.txt
# Creates document.txt.202609111723.bkp
```
