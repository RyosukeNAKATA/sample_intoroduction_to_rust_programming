use clap::{AppSettings, Parser};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Ryosuke",
    about = "Super awsome sample RPN calculator"
)]
struct Opts {
    /// Sete the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        // $B%U%!%$%k$r;XDj$7$J$+$C$?>l9g(B
        println!("No file is specified");
    }
}
