use clap::Parser;

/// TODO
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    file_type: String,
}

fn main() {
    let args = Args::parse();

    
    println!("{:?}", args);
}
