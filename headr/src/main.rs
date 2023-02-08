use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use clap::Parser;

/// Rust version of 'head'
#[derive(Parser)]
#[command(author, version, about)]
struct CliArgs {
    /// Input file(s)
    #[arg(value_name="FILES", default_value="-")]
    files: Vec<String>,

    /// Number of lines to display
    #[arg(
        short('n'),
        long("lines"),
        default_value="10",
        value_name="NUM",
        value_parser=clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// Number of bytes to display
    #[arg(
        short('c'),
        long("bytes"),
        value_name="NUM",
        conflicts_with("lines"),
        value_parser=clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>
}

type HeadrResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> HeadrResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}


fn run(args: CliArgs) -> HeadrResult<()> {
    let num_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate(){
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num == 0 {""} else {"\n"},
                        filename
                    );
                }

                if let Some(num_bytes) = args.bytes {
                    let mut handle = file.take(num_bytes);
                    let mut buffer = vec![0; num_bytes as usize];
                    let num_bytes_read = handle.read(&mut buffer)?;

                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..num_bytes_read])
                    );
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break
                        }
                        print!("{}", line);
                        line.clear();
                    }

                }
            }
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
