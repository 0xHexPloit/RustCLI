use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

type WcrResult<T> = Result<T, Box<dyn Error>>;

/// Rust version of 'wc'
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct CliArgs {
    /// Input file(s)
    #[arg(value_name="FILES", default_value="-")]
    files: Vec<String>,

    /// Show line count
    #[arg(
        short('l'),
        long("lines")
    )]
    lines: bool,

    /// Show word count
    #[arg(
        short('w'),
        long("words")
    )]
    words: bool,

    /// Show byte count
    #[arg(
        short('c'),
        long("bytes")
    )]
    bytes: bool,

    /// Show character count
    #[arg(
        short('m'),
        long("chars"),
        conflicts_with("bytes")
    )]
    chars: bool
}


#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_chars: usize,
    num_bytes: usize
}


fn count(mut file: impl BufRead) -> WcrResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_chars = 0;
    let mut num_bytes = 0;
    let mut line  = String::new();

    loop {
        let read_bytes = file.read_line(&mut line)?;
        if read_bytes == 0 {
            break;
        }

        num_lines += 1;
        num_bytes += read_bytes;
        num_chars += line.chars().count();
        num_words += line.split_whitespace().count();


        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_chars,
        num_bytes
    })
}


fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}



fn open(filename: &str) -> WcrResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}


fn run(args: CliArgs) -> WcrResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &
        args.files {
        match open(&filename) {
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, args.lines),
                        format_field(info.num_words, args.words),
                        format_field(info.num_bytes, args.bytes),
                        format_field(info.num_chars, args.chars),
                        if filename.as_str() == "-" { "".to_string() } else {format!(" {}", filename)}
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            },
            Err(e) => {
                eprintln!("{}: {}", filename, e);
            }
        }
    }

    if args.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, args.lines),
            format_field(total_words, args.words),
            format_field(total_bytes, args.bytes),
            format_field(total_chars, args.chars)
        )
    }

    Ok(())
}

fn main() {
    let mut args = CliArgs::parse();

    // Checking that some flags have been set otherwise setting the 'lines', 'words' and 'bytes'
    // flags to true
    if [args.lines, args.words, args.chars, args.bytes].iter().all(|item| item == &false) {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }


    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }

}
