mod cli;
mod r#const;

fn main() {
    let opt = cli::CommandLineParser::parse();
    println!("{:?}", opt);
}
