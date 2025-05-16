use std::process;

use prgf::ClInfo;

use prgf::run;

fn main() {
    let info = ClInfo::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = prgf::run(&info) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
    println!("{:?}", info)

}
