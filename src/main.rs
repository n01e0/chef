use clap::Parser;
use anyhow::Result;
use chef::encode::url;

#[derive(Parser)]
struct Args {
    #[clap(long="strict", short='s')] 
    strict: bool,
    #[clap(long="all", short='a')]
    all: bool,
    input: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", url::encode(args.input.as_bytes(), args.strict, args.all));

    Ok(())
}
