use clap::Parser;

/// Rust version of the 'echo' program
#[derive(Parser)]
#[command(about, version, author)]
struct CliArgs {
    /// Input text
    #[arg(required=true)]
    text: Vec<String>,

    /// Do not print new line
    #[arg(short('n'))]
    omit_newline: bool
}

fn main() {
    let args = CliArgs::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else {"\n"}
    );
}
