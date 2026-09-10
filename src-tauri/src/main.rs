//! setlist — bare binary. With no arguments it launches the Tauri app;
//! `setlist transpose <file> <semitones>` runs the CLI harness.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "transpose" {
        return cli_transpose(&args[2..]);
    }
    setlist_lib::run();
    ExitCode::SUCCESS
}

fn cli_transpose(args: &[String]) -> ExitCode {
    if args.len() != 2 {
        eprintln!("usage: setlist transpose <file> <semitones>");
        return ExitCode::from(2);
    }
    let path = &args[0];
    let semitones: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("error: <semitones> must be an integer, got {:?}", args[1]);
            return ExitCode::from(2);
        }
    };
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    print!(
        "{}",
        setlist_lib::chordpro::transpose_chart(&text, semitones)
    );
    ExitCode::SUCCESS
}
