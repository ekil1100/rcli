/// rcli csv -i input.csv -o output.csv -h -d ','
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(name = "csv", about = "csv file parser")]
    Csv(Csv),
}

#[derive(Parser, Debug)]
struct Csv {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long)]
    delimiter: char,

    #[arg(short, long)]
    header: bool,
}

fn main() {
    println!("Hello, world!");
}
