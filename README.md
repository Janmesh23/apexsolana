# Rust Project Setup Guide

This guide provides step-by-step instructions on how to install Rust, set up the required dependencies for your operating system, and initialize a new Rust project.


## 1. Installing Rust (rustup)

The recommended way to install Rust is via `rustup`, which is the official Rust toolchain installer.

### macOS and Linux
1. Open your terminal.
2. Run the following command:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. During the installation, you should generally proceed with the default installation (option `1`).
4. Once the installation is complete, apply the changes to your current shell session:
   ```bash
   source $HOME/.cargo/env
   ```

### Windows
1. Download `rustup-init.exe` from the official [rustup website](https://rustup.rs/).
2. Run the executable and follow the terminal prompts (selecting the default installation `1` is usually best).
3. Restart your terminal to ensure the Rust environment variables are loaded.

### Verifying the Installation
To confirm that Rust was installed successfully, run the following commands:

```bash
rustc --version
cargo --version
```
If these commands output version numbers, Rust is ready to use!

***

## 2. Setting Up a New Rust Project

Rust uses `cargo` as its package manager and build system. Setting up a new project is very straightforward.

### Creating a New Project
To create a new Rust project (an executable binary), run:

```bash
cargo new my_rust_project
cd my_rust_project
```

This creates a new directory named `my_rust_project` with the following structure:
- `Cargo.toml`: The configuration file containing your project's metadata and dependencies.
- `src/main.rs`: The entry point for your application.
- `.git`: A pre-initialized Git repository.

*(If you are building a library instead of an application, use `cargo new my_rust_project --lib` instead).*

### Building and Running the Project
- **To build the project**:
  ```bash
  cargo build
  ```
  *(This compiles the project and places the executable in `target/debug/`)*

- **To run the project**:
  ```bash
  cargo run
  ```
  *(This correctly builds the project if needed and runs the binary immediately)*

- **To check if the project compiles (fast validation without producing a final binary)**:
  ```bash
  cargo check
  ```

- **To build the project for production (optimized)**:
  ```bash
  cargo build --release
  ```
  *(This takes longer to compile but produces a much faster executable, located in `target/release/`)*



## 3. Managing Dependencies

To add dependencies to your project, you can manually open the `Cargo.toml` file and add them to the `[dependencies]` section:

```toml
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
```
Then, run `cargo build` to download and compile the new dependencies.
