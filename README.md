<div align="center"> 

[![Rust Building](https://github.com/Walker-00/bitfuck/actions/workflows/rust.yml/badge.svg)](https://github.com/Walker-00/bitfuck/actions/workflows/rust.yml)
[![Clippy Code Analyze](https://github.com/Walker-00/bitfuck/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Walker-00/bitfuck/actions/workflows/rust-clippy.yml)

</div>

# bitfuck
Just an esoteric language interpreter with Rust

## [Info]

- BitFuck is just an esoteric programming language like BrainFuck
- This interpreter is written in Rust
- BitFuck has only 10 keywords, 1 is for comment
- In this Interpreter implementation default Memory is 99999 bytes

## [Require]

- [Rust](https://rust-lang.org)

## [Build and Install]

```sh
$ git clone https://github.com/Walker-00/bitfuck
$ cd bitfuck
$ cargo install --path .
```

## [Usage]

```sh
$ bitfuck <source file> 
```

## [keywords-Docs]

| Keywords | Functionality                                                       |
|----------|---------------------------------------------------------------------|
| ptp      | Point To Next Byte of Memory.                                       |
| ptd      | Point To Previous Byte of Memory.                                   |
| inc      | Increase 1 bit to Pointed Byte.                                     |
| dec      | Decrease 1 bit to Pointed Byte.                                     |
| out      | Write The Pointed Value To Output.                                  |
| ask      | Read a Character To Pointed Byte.                                   |
| lps      | If Pointed Byte is 0 continue execution till it find `lpe` keyword. |
| lpe      | If Pointed Value is 0 stop the execution, else continue execution.  |
| deb      | Keyword use To Print the Debug info to output.                      |
| !        | Keyword to Write comments                                           |
