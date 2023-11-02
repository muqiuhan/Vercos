/// Copyright (C) 2023 Muqiu Han
mod cli;
mod commands;
mod r#const;
mod error;
mod repo;

#[macro_use]
extern crate log;
extern crate colog;

fn main() {
    colog::init();
    let args = cli::CommandLineParser::parse();
    commands::command(&args)
}
