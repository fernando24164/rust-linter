# Rust linter

A small linter project made with Rust. Just for learning purposes 📖📓📚🎓

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)

## About <a name = "about"></a>

A small linter to detect extra commas in Marathon templates files


## Getting Started <a name = "getting_started"></a>

Some cases your text editor can add extra commas to last line in configuration like this.

```
{
    "id": "basic-0",
    "cmd": "while [ true ] ; do echo 'Hello Marathon' ; sleep 5 ; done",
    "cpus": 0.1,
    "mem": 10.0,
    "instances": 1,
}
```

To check it, I made this naive linter in Rust

```
rust-linter <marathon.json>
```

This will return the exact line with extra comma

```
"Error in line 6, extra comma"
```

### Prerequisites

Install Rust in your computer

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing

Installing in Linux

```
cargo test
cargo build --release
cp rust-linter /usr/local/bin
chmod +x /usr/local/bin/rust-linter
```

## Usage <a name = "usage"></a>

```
rust-linter <marathon-template.json>
```
