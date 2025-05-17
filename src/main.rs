use std::process;

use prgf::ClInfo;

fn main() {
    let info = ClInfo::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = info.run() {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }

}
