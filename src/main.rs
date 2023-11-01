mod cli;
mod r#const;
mod error;
mod repo;
fn main() {
    let opt = cli::CommandLineParser::parse();
    println!("{:?}", opt);
}
