//! `sharp` — a very small shell.
//!
//! It reads one command per line from standard input, splits the line on
//! whitespace and runs the first word as a program, passing the rest as
//! arguments. The only builtin is `cd`, which changes the working directory of
//! the shell process itself. Everything else is looked up on `PATH` and handed
//! to the operating system.
//!
//! There is deliberately no quote handling, no globbing, no pipes and no
//! redirections: this is a 100-line demonstration of `std::process::Command`,
//! not a bash replacement.
//!
//! # Example
//!
//! ```text
//! $ echo hello world
//! hello world
//! $ nosuchprogram
//! sh: nosuchprogram: No such file or directory (os error 2)
//! $ exit
//! ```

use std::env;
use std::io::{self, BufRead, Write};
use std::process::Command;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use rustyline::history::History;


/// String printed in front of every input line.
const PROMPT: &str = "$ > ";

/// Reported as the exit status of a child that died from a signal rather than
/// returning a code of its own.
const KILLED_BY_SIGNAL: i32 = -1;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();

    let mut rl = DefaultEditor::new().expect("failed to initialize the line editor");

    let history_path = env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(".sharp_history"));

    if let Some(path) = &history_path {
        let _ = rl.load_history(path);
    }

    loop {
        print!("{PROMPT}");
        io::stdout().flush().expect("failed to flush stdout");

        // Reuse the same buffer every iteration so the loop does not allocate
        // a new String per prompt.
        line.clear();
        let line = match rl.readline(PROMPT) {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(error) => {
                eprintln!("sharp: {error}");
                break;
            }
        };

        let words: Vec<&str> = line.split_whitespace().collect();
        let Some((command, args)) = words.split_first() else {
            // Blank line, just print the prompt again.
            continue;
        };

        if !line.trim().is_empty() {
            let _ = rl.add_history_entry(line.as_str());
        }

        if *command == "exit" {
            break;
        }

        if *command == "cd" {
            if let Err(error) = change_directory(args.first().copied()) {
                eprintln!("sh: cd: {error}");
            }
            continue;
        }

        if *command == "history" {
            print_history(&rl);
            continue;
        }

        match Command::new(command).args(args).status() {
            Ok(status) if !status.success() => {
                eprintln!("exit {}", status.code().unwrap_or(KILLED_BY_SIGNAL));
            }
            Ok(_) => {}
            Err(error) => eprintln!("sh: {command}: {error}"),
        }
    }

    if let Some(path) = &history_path {
        rl.save_history(path);
    }
}

/// Point the shell at a different working directory.
///
/// A bare `cd` goes to `$HOME`. Returns the [`io::Error`] from the operating
/// system when the directory cannot be entered.
fn change_directory(path: Option<&str>) -> io::Result<()> {
    let target = match path {
        Some(path) => path.to_owned(),
        None => env::var("HOME").unwrap_or_default(),
    };
    env::set_current_dir(target)
}

fn print_history(rl: &DefaultEditor) {
    let history = rl.history();
    if history.is_empty() {
        println!("(history is empty)");
        return;
    }
    println!("history");
    for (i, entry) in history.iter().enumerate() {
        println!("  {}  {}", i + 1, entry);
    }
}
