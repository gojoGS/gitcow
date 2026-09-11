use std::io::{BufRead, BufReader, Write};
use std::{
    fs::{self, File, OpenOptions},
    path,
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init { path: String },
    Status,
    Create { branch_name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { path } => init(&path),
        Command::Status => status(),
        Command::Create { branch_name } => create_worktree(&branch_name),
    }
}

fn create_worktree(branch_name: &str) {
    let workdir_path = path::Path::new(branch_name);

    if workdir_path.exists() {
        panic!("can't create worktree, as file/directory already exists");
    }

    fs::create_dir(workdir_path).unwrap();
    println!("git worktree add {branch_name}");

    let mut cowfile = OpenOptions::new()
        .create(false)
        .append(true)
        .write(true)
        .open(".cow")
        .expect("gitcow hasn't been initialized");
    writeln!(cowfile, "?{branch_name}").expect("couldn't register cowrepo");
}

fn status() {
    let cowfile = File::open(".cow").unwrap();
    let reader = BufReader::new(cowfile);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{line}")
    }
}

fn init(path: &str) {
    let mut file = File::create_new(".cow").expect("gitcow already initialized");

    writeln!(file, "!{path}").unwrap();

    println!("git clone to {path}");
}
