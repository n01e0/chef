use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
struct Args {
    #[clap(long="strict", short='s')] 
    strict: bool,
    #[clap(long="all", short='a')]
    all: bool,
    input: String,
}

fn encode(args: Args) -> String {
    let mut encoded = String::new();
    for c in args.input.bytes() {
        if args.all {
            encoded.push_str(&format!("%{:02X}", c))
        } else {
            match c {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    if args.strict && b"-_.~".contains(&c) {
                        encoded.push_str(&format!("%{:02X}", c))
                    } else {
                        encoded.push(c as char)
                    }
                }
                _ => encoded.push_str(&format!("%{:02X}", c)),
            }
        }
    }
    encoded
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", encode(args));

    Ok(())
}
