use clap::Parser;
use hash_finder::args::Args;

fn main() {
    let args = Args::parse();

    // Validate inputs
    if args.zeros == 0 || args.count == 0 {
        eprintln!("Error: N and F must be positive integers.");
        std::process::exit(1);
    }
}
