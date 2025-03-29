## Day1

## Install Rust & Cargo (rustup)

    Rust is a modern systems programming language designed for safety, performance, and concurrency. It eliminates memory safety issues found in languages like C and C++.

    Cargo is Rust’s package manager and build system. It simplifies dependency management, compiling, and running Rust programs.

    rustup is the recommended tool to install and manage Rust versions and components. It ensures you always have the latest stable version of Rust.

## ✅ Installation Steps:

1. Open a terminal.

2. Run the command:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Follow the on-screen instructions to complete installation.

4. Restart your terminal and verify installation:

```
rustc --version
cargo --version
```

## create rust project using cargo

1. Run this command

```
cargo init
```

2. To buid the project

```
cargo build
```

3. To run the project

```
cargo run
```
