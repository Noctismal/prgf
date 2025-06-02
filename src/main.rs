use std::process;

use prgf::ClInfo;

fn main() {
    // build the command line arguments
    let info = ClInfo::build().unwrap_or_else(|err| {
        // if an err is encountered print and exit program
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("{:?}", info);

    if let Err(err) = info.run() {
        // if err is encountered whie running print and exit
        eprintln!("Application error: {err}");
        process::exit(1);
    }

}
