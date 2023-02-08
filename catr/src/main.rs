use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
/// Rust version of 'cat'
struct CliArgs {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Print number lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    /// Print number lines (expect for blank lines)
    #[arg(
        short('b'),
        long("number-nonblank")
    )]
    number_nonblank_lines: bool
}

type CatrResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> CatrResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn run(args: CliArgs) -> CatrResult<()> {
    for filename in args.files {
        match open(&filename) {
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;

                    if args.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!()
                        } else {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            },
            Err(e) => eprintln!("{}: {}", filename, e)
        }
    }
    Ok(())
}


fn main() {
    let args = CliArgs::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
