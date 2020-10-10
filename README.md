![CI](https://github.com/tversteeg/sprite-gen/workflows/CI/badge.svg)

# [sprite](https://tversteeg.itch.io/sprite) (Executable)

[![Cargo](https://img.shields.io/crates/v/sprite.svg)](https://crates.io/crates/sprite) [![License: GPL-3.0](https://img.shields.io/crates/l/sprite.svg)](#license) [![Downloads](https://img.shields.io/crates/d/sprite.svg)](#downloads)

## Run

On Linux you need the [gtk-rs dependencies](https://gtk-rs.org/docs/requirements.html) to compile:

    cargo install sprite
    sprite

This should produce the following window:

![Sprite](img/sprite.png?raw=true)

# sprite-gen (Library)

A Rust library for procedurally generating 2D sprites. Port of https://github.com/zfedoran/pixel-sprite-generator

[![Cargo](https://img.shields.io/crates/v/sprite-gen.svg)](https://crates.io/crates/sprite-gen) [![License: GPL-3.0](https://img.shields.io/crates/l/sprite-gen.svg)](#license) [![Downloads](https://img.shields.io/crates/d/sprite-gen.svg)](#downloads)

### [Documentation](https://docs.rs/sprite-gen/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sprite-gen = "0.2"
```

And this to your crate root:

```rust
extern crate sprite_gen;
```

### Run the example

On Linux you need the `xorg-dev` package as required by `minifb` -- `sudo apt install xorg-dev`

    cargo run --example minifb

This should produce the following window:

![Example](img/example.png?raw=true)
