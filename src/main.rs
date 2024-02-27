#![allow(clippy::uninlined_format_args)]
use std::{
    env::current_exe,
    os::unix::process::CommandExt,
    process::{exit, Command},
};

use axoupdater::{AxoUpdater, AxoupdateError};
use clap::Parser;
use unicode_width::UnicodeWidthStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    msg: String,
}

fn main() -> Result<(), AxoupdateError> {
    if AxoUpdater::new_for("axolotlsay").load_receipt()?.run()? {
        eprintln!("");
        eprintln!("New version installed! Restarting...");

        let exe = current_exe().unwrap();
        let mut args: Vec<String> = std::env::args().collect();

        Command::new(exe).args(&mut args[1..]).exec();

        // If we reached here, it means there was an error running it
        eprintln!("Failed to exec new program!");
        exit(1);
    }

    let args = Args::parse();
    let msg = &args.msg;
    let count = UnicodeWidthStr::width(args.msg.as_str());
    let dashes = "-".repeat(count + 2);
    println!("         +{}+", dashes);
    println!("         | {} |", msg);
    println!("         +{}+", dashes);
    println!("        /");
    println!("≽(◕ ᴗ ◕)≼");

    Ok(())
}
