# Setup environment

Rust is installed through rustup, a command line tool for managing Rust versions and associated tools.

```sh
curl https://sh.rustup.rs -sSf | sh
```

To update Rust

```sh
rustup update
```

[rustc](https://doc.rust-lang.org/rustc/index.html) is the compiler for the Rust programming language, provided by the project itself. Most Rust programmers don't invoke rustc directly, but instead do it through [Cargo](https://doc.rust-lang.org/cargo/index.html).

## Cargo

Cargo is the Rust package manager. Cargo downloads your Rust package’s dependencies, compiles your packages, makes distributable packages, and uploads them to [crates.io](https://crates.io)

### Create a project

Clone this repository

```sh
git clone git@github.com:Cutii/rustii.git && cd rustii
```

To build your project run

```sh
cargo build # build
cargo run #build & run
cargo test #run unit tests
cargo build --release #build in production mode
```

If you would create a new project from scratch, you could do :

```sh
cargo new my_project
```

Or if you want to creat a lib

```sh
cargo new my_project --lib
```

You can also initialize a project from an existing repository

```sh
cargo init
```

You can notice the presence of a `cargo.toml` file which is call the manifest. Every manifest file consists of one or more sections : package description, depencies management, configuration, ...

### Tools

- Install [rust plugin for your IDE](https://www.rust-lang.org/tools)
- Install [TOML syntax for your IDE](https://github.com/toml-lang/toml/wiki#editor-support)
