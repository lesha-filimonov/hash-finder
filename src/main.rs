use clap::Parser;
use hash_finder::args::Args;
use hash_finder::finder::find_hashes;

fn main() {
    let args = Args::parse();

    if args.zeros == 0 || args.count == 0 {
        eprintln!("Error: N and F must be positive integers.");
        std::process::exit(1);
    }

    let results = match find_hashes(args.zeros, args.count) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    for result in &results {
        println!("{}  \"{}\"", result.number, result.hash);
    }
}
