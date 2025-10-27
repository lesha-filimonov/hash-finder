use clap::Parser;

#[derive(Parser)]
#[command(about = "Finds SHA-256 hashes of consecutive integers ending with N zeros")]
pub struct Args {
    #[arg(short = 'N', long, help = "Number of trailing zeros in hash")]
    pub zeros: usize,
    #[arg(short = 'F', long, help = "Number of results to find")]
    pub count: usize,
}
