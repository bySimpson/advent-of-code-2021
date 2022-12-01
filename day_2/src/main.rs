use clap::Parser;
use command::{Command, CommandType};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    vec,
};

mod command;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;

    let mut commands: Vec<Command> = vec![];

    for line in reader.lines() {
        let command = Command::new(line?);
        match command.command_type {
            CommandType::Forward => horizontal += command.amount,
            CommandType::Down => depth += command.amount,
            CommandType::Up => depth -= command.amount,
        };
        commands.push(command);
    }
    println!("Horizontal x depth: \t\t{}", horizontal * depth);

    let mut corrected_depth = 0;
    let mut aim = 0;
    for c_command in commands {
        match c_command.command_type {
            CommandType::Forward => corrected_depth += aim * c_command.amount,
            CommandType::Down => aim += c_command.amount,
            CommandType::Up => aim -= c_command.amount,
        };
    }

    println!(
        "Horizontal x corrected depth: \t{}",
        corrected_depth * horizontal
    );

    Ok(())
}
