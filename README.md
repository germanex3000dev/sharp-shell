# sharp

A very small shell written in Rust.

`sharp` reads one command per line, splits it on whitespace and runs the first
word as a program with the remaining words as arguments. It is about a hundred
lines long and exists to show how far `std::process::Command` gets you. So 
basically, it's pointless, again. But remember, this is just a small first version. 
It will get many cool features soon. 

## Features

- Runs any program found on `PATH`, passing arguments through untouched
- `cd` builtin, including a bare `cd` to go to `$HOME`
- `exit` builtin, plus Ctrl-D to end the session
- Prints the same exit code a real shell would for a failing command
- No dependencies

## Not supported

No quote handling, no globbing, no pipes, no redirections, no job control, no
built-in `PATH` search. Use a real shell for real work.

## Requirements

Rust 1.85 or newer (the crate uses edition 2024).

## Build

```sh
cargo build --release
```

The binary lands in `target/release/sharp`.

## Run

```sh
./target/release/sharp
```

Or install it onto your `PATH` with:

```sh
cargo install --path .
```

## Usage

```console
$ ./target/release/sharp
> echo hello world
hello world
> cd /tmp
> pwd
/tmp
> nosuchprogram
sh: nosuchprogram: No such file or directory (os error 2)
> false
exit 1
> exit
```

## How it works

1. Print the `> ` prompt and flush, otherwise the prompt gets buffered and only
   shows up after the first command finishes.
2. Read one line. End of file means the user pressed Ctrl-D.
3. Split the line on whitespace. An empty line just reprints the prompt.
4. `exit` ends the loop, `cd` is handled in-process because a real `cd` binary
   cannot change the shell's own directory.
5. Anything else goes to `Command::new(...).status()`, which waits for the child
   and gives back its exit status.

## License

[MIT](LICENSE) © Germanex3000
