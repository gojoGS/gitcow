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
    Init { clone_url: String, path: String },
    Status,
    Create { branch_name: String, path: String },
    Remove { path: String },
    Bail,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { path, clone_url } => init(&clone_url, &path),
        Command::Status => status(),
        Command::Create { branch_name, path } => create_worktree(&branch_name, &path),
        Command::Remove { path } => todo!(),
        Command::Bail => todo!(),
    }
}

fn create_worktree(branch_name: &str, path: &str) {
    let workdir_path = path::Path::new(path);

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
    writeln!(cowfile, "?{branch_name} {path}").expect("couldn't register cowrepo");
}

fn status() {
    let cowfile = File::open(".cow").unwrap();
    let reader = BufReader::new(cowfile);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{line}")
    }
}

fn init(clone_url: &str, path: &str) {
    let mut file = File::create_new(".cow").expect("gitcow already initialized");

    writeln!(file, "!{path}").unwrap();

    println!("git clone {clone_url} {path}");
}
