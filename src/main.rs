use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    source_code: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if let Some(source_code) = &args.source_code {
        println!("Running rlox interpreter on {:?}", source_code);
    } else {
        println!("Running rlox prompt");
    }
}
