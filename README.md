
# RTree

> A lightweight `tree` + `ls -lh` inspired CLI tool written in Rust.

RTree prints directory trees with optional file sizes, Unix-style permissions, and smart colorized output. It is implemented as a single-binary CLI tool using `clap` for argument parsing and `owo-colors` for styling.

---

## Features

* Tree-style directory structure
* Recursive directory size calculation
* Optional Unix-style permissions column
* Optional human-readable file sizes
* Smart file-type coloring
* Automatic terminal color detection
* Manual `--no-color` override
* Depth limiting
* Directory-only mode
* Multiple ignore rules
* Short and long flags
* Unix executable detection (on Unix systems)
* Cross-platform support (Windows supported, simplified permissions)

---

## Installation

### Clone

```bash
git clone https://github.com/yourusername/rtree.git
cd rtree
```

(Optional) Move to PATH:

```bash
sudo mv target/release/rtree /usr/local/bin/
```

---

## Usage

```bash
rtree [OPTIONS] <path>
```

### Arguments

| Argument | Description     |
| -------- | --------------- |
| `<path>` | Path to inspect |

---

### Options

| Flag                  | Description                                             |
| --------------------- | ------------------------------------------------------- |
| `-s`, `--size`        | Show file sizes (directories show recursive total size) |
| `-p`, `--permissions` | Show Unix-style permissions                             |
| `-d`, `--depth <n>`   | Limit recursion depth                                   |
| `--only-dirs`         | Show only directories                                   |
| `--ignore <name>`     | Ignore specific file or directory (repeatable)          |
| `--no-color`          | Disable colored output                                  |

---

## Examples

Show sizes and permissions:

```bash
rtree . -s -p
```

Limit recursion depth:

```bash
rtree . --depth 2
```

Ignore multiple entries:

```bash
rtree . --ignore target --ignore .git
```

Show only directories:

```bash
rtree . --only-dirs
```

Disable colors:

```bash
rtree . --no-color
```

---

## Output Behavior

### Tree Rendering

* Uses `├──`, `└──`, and `│` connectors
* Entries are sorted alphabetically by path
* Recursively traverses directories

### File Sizes

* Files show direct size
* Directories show total recursive size
* Human-readable units (B, K, M, G, T)

### Permissions

* Full Unix-style permissions on Unix systems (`drwxr-xr-x`)
* Simplified read-only/read-write display on non-Unix platforms

### Coloring Rules (when enabled)

* Directories → **Blue + Bold**
* Symlinks → Cyan
* Executables (Unix) → Green
* Rust files (`.rs`) → Bright magenta
* Images (`png`, `jpg`, `jpeg`, `gif`, `webp`) → Yellow
* Hidden files (`.` prefix) → Dimmed
* Connectors → Bright black

Colors are automatically disabled if stdout is not a terminal, unless explicitly overridden.

---

## Dependencies

* `clap` – CLI argument parsing
* `owo-colors` – Terminal styling

---

## Platform Notes

* Full Unix permissions and executable detection on Unix systems
* Windows builds supported (permissions simplified)
* Uses `std::fs::symlink_metadata` to detect symlinks correctly

---

## Implementation Notes

* Single-file implementation
* Direct recursive traversal (no async or parallelism)
* No external tree-building abstraction
* Designed for simplicity and readability


## Acknowledgment

This project was developed as a learning exercise in Rust systems programming.  
AI assistance (ChatGPT) was used to explore architectural improvements, optimization strategies, and design refinements. All implementation decisions and final structure were reviewed and integrated manually.
