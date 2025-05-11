use clap::Parser;

/// Create a basic programming file
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// name of the language you wnat to make a file for
    #[arg(short, long)]
    file_type: String,    
}

impl Args {
    // builds the arguments into the struct
    pub fn build() -> Args {
        Args::parse()
    }
}