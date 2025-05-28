# tree

A simple command-line utility written in Rust to display directory structures in a tree-like format, similar to the UNIX `tree` command. It supports options for maximum depth and showing hidden files.

---

## Features

* Recursive directory traversal using Depth-First Search (DFS)
* Customizable maximum depth (`--depth`, `-d`)
* Option to include hidden files (`--all`, `-a`)
* Clean Unicode tree connectors (`├`, `└`)
* Safe, efficient implementation leveraging Rust's ownership and borrow checker
* Comprehensive error handling with `Result` and `Option`
* Command-line argument parsing via `clap`
* Unit tests for core functionality

---

## Installation

Ensure Rust and Cargo are installed (see [https://rustup.rs](https://rustup.rs)). Then clone and build:

```bash
# Clone the repository
git clone https://github.com/yourusername/rtree.git
cd rtree

# Build in debug mode\ ncargo build

# (Optional) Build optimized release
cargo build --release
```

---

## Usage

```bash
# Print the full directory tree
cargo run -- ./testdir

# Limit depth to 2 levels
cargo run -- ./testdir --depth 2

# Include hidden files
cargo run -- ./testdir --all

# Combine options
cargo run -- ./testdir -d 3 -a
```

After building, run:

```bash
./target/release/rtree ./testdir --depth 2 --all
```

### Example

Given this directory structure:

```
testdir/
├── file1.txt
├── .hidden.txt
└── subdir/
    └── nested.txt
```

* **Without hidden files, full depth:**

  ```bash
  rtree testdir
  ```

  Output:

  ```
  ```

testdir
├── file1.txt
└── subdir
└── nested.txt

````

- **With hidden files, depth limit 1:**

  ```bash
  rtree testdir --all --depth 1
````

Output:

```
testdir
├── file1.txt
├── .hidden.txt
└── subdir
```

---

## Code Overview

* **`main.rs`**

  * Uses `clap::Parser` to parse command-line arguments (`path`, `depth`, `all`).
  * Constructs a `Path` from the input string and calls `print_tree`.

* **`print_tree(path: &Path, prefix: String, depth: usize, show_hidden: bool)`**

  * Reads directory entries via `std::fs::read_dir`.
  * Filters out hidden files unless `show_hidden` is set.
  * Sorts entries alphabetically.
  * Prints each entry with appropriate tree connectors and prefix.
  * Recursively calls itself for subdirectories, decreasing `depth` until zero.

* **Ownership & Borrowing**

  * `path` is passed as `&Path`, borrowing the reference.
  * `prefix` is passed by value (`String`) and dropped automatically when out of scope.

* **Error Handling**

  * Uses `Result` and the `?` operator to propagate I/O errors.
  * Provides user-friendly error messages on failure.

* **Unit Tests**

  * Defined under `tests/` or inline using Rust’s built-in testing framework.
  * Validate depth limiting, hidden file filtering, and error cases.

---

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Implement your changes and add unit tests.
4. Commit and push (`git push origin feature-name`).
5. Open a Pull Request.

Please ensure tests pass (`cargo test`) and code is formatted (`cargo fmt`).

---
![스크린샷 2025-05-29 022856](https://github.com/user-attachments/assets/5d932b85-2edd-496c-8921-2d9e22595c92)


