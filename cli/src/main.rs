use std::fs::File;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, short)]
    input: String,

    #[arg(long, short)]
    output: String,

    #[arg(long, short)]
    time: humantime::Duration,
}

fn main() {
    let cli = Cli::parse();
    let input = File::open(&cli.input).unwrap();
    let mut output = File::create(&cli.output).unwrap();
    core::process(input, &mut output, cli.time.into());
}
