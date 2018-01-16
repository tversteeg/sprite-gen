# sprite-gen

A Rust library for procedurally generating 2D sprites. Port of https://github.com/zfedoran/pixel-sprite-generator

[![Build Status](https://travis-ci.org/tversteeg/sprite-gen.svg?branch=master)](https://travis-ci.org/tversteeg/sprite-gen) [![Cargo](https://img.shields.io/crates/v/sprite-gen.svg)](https://crates.io/crates/sprite-gen) [![License: GPL-3.0](https://img.shields.io/crates/l/sprite-gen.svg)](#license) [![Downloads](https://img.shields.io/crates/d/sprite-gen.svg)](#downloads)

### [Documentation](https://docs.rs/sprite-gen/)

## Executable

## Run

On Linux you need the `xorg-dev` package as required by `minifb` -- `sudo apt install xorg-dev`

    cargo run --release

# Library

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sprite-gen = "0.1"
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
