# sharp

A very small shell written in Rust.

> **Status: very early.** sharp is at version `0.1.1`. It reads commands, runs them, and
> remembers history — but there is no quote handling, no globbing, no pipes, no
> redirections, and no job control. Expect things to break and the API to change.

> Small note from me: This project has no code written by AI, I typed everything by hand,
> but I used AI to explain stuff to me, since I'm very new to Rust

## Goals

- **Small.** A tight, readable codebase you can actually hold in your head.
- **Minimalistic.** No bloat, no kitchen-sink feature set for its own sake.
- **Modern.** Written from scratch in Rust, using `std::process::Command` and `rustyline`.
- **Featureful.** The long-term goal is a genuinely usable interactive shell with many QoL features — see [Roadmap](#roadmap).

## Current Features

- Runs any program found on `PATH`, passing arguments through untouched
- `cd` builtin, including a bare `cd` to go to `$HOME`
- `exit` builtin, plus `Ctrl-D` to end the session
- Prints the same exit code a real shell would for a failing command
- **Command history** via `rustyline` — up/down arrows navigate, `Ctrl-R` searches
- No external dependencies beyond `rustyline` (single crate)

## Keybindings

Bindings are provided by `rustyline`'s default emacs mode (and vi mode with `Esc`):

| Key          | Action                            |
| ------------ | --------------------------------- |
| `↑` / `↓`    | Previous / next history entry     |
| `Ctrl+P` / `Ctrl+N` | Previous / next history entry (emacs) |
| `Ctrl+R`     | Reverse incremental history search |
| `Ctrl+A` / `Ctrl+E` | Beginning / end of line         |
| `Ctrl+K`     | Kill to end of line               |
| `Ctrl+U`     | Kill whole line                   |
| `Ctrl+W`     | Kill previous word                |
| `Ctrl+L`     | Clear screen                      |
| `Tab`        | No completion yet (passthrough)   |
| `Ctrl+D`     | Exit shell (on empty line)        |
| `Ctrl+C`     | Cancel current line               |

## Current Progress

> This section is kind of like a Timestamped journal. I like to keep notes.
> For the bigger picture, see [Goals](#goals) and [Roadmap](#roadmap).

### Lates(small) 25.10.2026, 14:08: command history file

sharp has now a command history file for persistent history across sessions. It uses Rustylines
builtin `save_history()` function to save the command history to `~/.sharp_history` on exit.

### Latest(major) 25.10.2026, 14:30: command history with rustyline

sharp now has persistent, editable command history using the `rustyline` crate. This replaces
the bare `stdin.lock().read_line()` loop with a proper line editor.

- **`DefaultEditor`**. Zero-config line editor that works out of the box on Linux/macOS/Windows.
- **History persistence**. History is saved to `~/.local/share/sharp/history.txt` automatically
  (respects `XDG_DATA_HOME`). Survives restarts.
- **Incremental search**. `Ctrl+R` searches backwards through history; `Ctrl+S` goes forward.
- **In-memory deduplication**. Empty lines are not added to history (`add_history_entry` is
  guarded by `!line.trim().is_empty()`).
- **Graceful error handling**. `ReadlineError::Interrupted` (Ctrl+C) and `ReadlineError::Eof`
  (Ctrl+D) both exit cleanly. Other errors print to stderr and break the loop.
- **Prompt integration**. The `$ > ` prompt is passed to `readline()` so the editor handles
  redisplay, cursor position, and multiline editing automatically.

### Latest(small) 20.10.2026, 11:15: proper exit code reporting

Non-zero exit codes from child processes are now printed as `exit <code>`, matching POSIX shell
behaviour. If a child dies from a signal, `exit -1` is printed (see `KILLED_BY_SIGNAL` const).

### Latest(small) 18.10.2026, 09:45: cd builtin with $HOME fallback

`cd` without arguments changes to `$HOME` (via `env::var("HOME")`). Errors from `chdir`
propagate as `io::Error` and are printed with the `sh: cd:` prefix.

### Latest(major) 15.10.2026, 16:20: first working command execution loop

The initial REPL: print prompt, read line, split on whitespace, execute via
`Command::new(...).status()`, repeat. Only builtin was `exit`. No history, no editing.

## Known Limitations

Deliberately minimal for a first cut, but worth knowing:

- **No quote handling.** `echo "hello world"` passes the quotes as literal arguments.
- **No globbing.** `echo *.rs` does not expand.
- **No pipes or redirections.** `|`, `>`, `<`, `>>`, `2>`, `&>` are all treated as literal arguments.
- **No job control.** `Ctrl+Z`, `bg`, `fg`, `jobs` do not exist.
- **No tab completion.** `Tab` inserts a literal tab character (rustyline default).
- **No `$PATH` search builtin.** Relies on the OS `execvp` semantics via `Command::new`.
- **No scripting support.** No variables, conditionals, loops, or functions.
- **No configuration file.** Prompt, history path, keybindings are hardcoded.
- ~~**History file location.** Follows XDG spec (`~/.local/share/sharp/history.txt`) but~~
  ~~creates parent dirs lazily on first write — first run on a fresh system may print a~~
  ~~harmless "could not create history dir" error if permissions are weird.~~
  LIE! History file now added.

## Requirements

- Rust 1.85 or newer (edition 2024)
- `rustyline` crate (pulled in via Cargo)

## Building

With Cargo:

```sh
cargo build --release
```

The binary lands in `target/release/sharp`.

## Running

```sh
./target/release/sharp
```

~~Or install it onto your `PATH`:~~


~~cargo install --path .~~

`cargo install --path .` Somehow doesn't work anymore, so idk, leave it out.

If you want to still install to your `PATH`, copy the compiled binary manually to your
`PATH` (Either only your user, `~/.local/bin/` or all users `/usr/bin/`)


Then just run:

```sh
sharp
```

## Usage

```console
$ sharp
$ > echo hello world
hello world
$ > cd /tmp
$ > pwd
/tmp
$ > nosuchprogram
sh: nosuchprogram: No such file or directory (os error 2)
$ > false
exit 1
$ > exit
```

Press `↑` / `↓` to navigate history, `Ctrl+R` to search.

## How it works

1. Create a `rustyline::DefaultEditor` (loads history from disk automatically).
2. Loop: call `readline("$ > ")` — this handles all line editing, history navigation,
   and rendering.
3. On `Ok(line)`: if non-empty, add to history via `add_history_entry()`.
4. Split on whitespace. First word is the command; rest are arguments.
5. `exit` breaks the loop. `cd` calls `env::set_current_dir()` in-process.
6. Everything else: `Command::new(command).args(args).status()` — waits for child,
   prints `exit <code>` on non-zero.

History is flushed to disk on editor drop (end of `main`).

## Roadmap

Completely not in order — this is a wish list, not a commitment:

- [ ] Quote handling (single, double, backslash escapes)
- [ ] Globbing / filename expansion
- [ ] Pipes (`|`) and redirections (`>`, `<`, `>>`, `2>`, `&>`)
- [ ] Job control (`Ctrl+Z`, `bg`, `fg`, `jobs`, `&`)
- [ ] Tab completion (executables, paths, builtins)
- [ ] Shell variables (`$VAR`, `${VAR}`, `$?`, `$$`, `$!`)
- [ ] Builtins: `export`, `unset`, `alias`, `unalias`, `history`, `pwd`, `true`, `false`
- [ ] Configuration file (`~/.config/sharp/config.toml` or similar)
- [ ] Script execution (`sharp script.sh`)
- [ ] Functions and control flow (`if`, `for`, `while`, `case`)
- [ ] Vi mode toggle (`set -o vi` / `set -o emacs`)

## Contributing

This is a personal project and contributions are not being solicited right now.
Patches, ideas and bug reports are still welcome once the codebase stabilises
a little — see the [issue tracker](https://github.com/Germanex3000/sharp/issues).

## License

Released under the [MIT License](LICENSE). Copyright (c) 2026 Germanex3000.
