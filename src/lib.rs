use std::{fs, io};
use std::fmt::Display;

use clap::Parser;


/// Create a basic programming file
#[derive(Parser, Debug)]
#[command(version, after_help = "Supported languages\n\tc\n\trust")]
struct Args {
    /// name of the language you wnat to make a file for
    #[arg(short, long)]
    file_type: String,    
}

impl Args {
    // builds the arguments into the struct
    fn build() -> Args {
        Args::parse()
    }
}

// holds information that the program needs to 
// use such as the contents that need to go into the file
#[derive(Debug)]
pub struct ClInfo {
    args: Args,
    extension: String,
    contents: Vec<String>
}

impl ClInfo {
    fn build_contents(file_type: &str) -> Result<Vec<String>, io::Error> {
        // contents should be in this file, if not return ft_file_null
        let file = fs::read_to_string("ft_types.txt")?;
        
        let ft: String  = "// ".to_owned() + file_type;

        // holds the contents
        let mut contents: Vec<String> = vec![];

        let mut flag = false;

        for line in file.lines() {
            if line == ft {
                flag = true;
                continue;
            }

            if flag == true {
                if line.contains("//") && line != ft {
                    break;
                }
                contents.push(line.to_string());
            }
        }
        Ok(contents)
    }

    fn build_extension(file_type: &str) -> Result<String, io::Error> {
        let file = fs::read_to_string("ft_extensions.txt")?;
        
        let ft: String = "// ".to_owned() + file_type;

        let mut flag = false;

        let mut extension = String::new();
    
        println!("{file_type}");

        for line in file.lines() {
            if line == ft {
                flag = true;
                continue;
            }
            
            if flag {
                extension = line.to_string();
                break;
            }
        }

        return Ok(extension)
    }

    pub fn build() -> Result<ClInfo, io::Error> {
        // get command line arguments
        let args = Args::build();

        let contents: Vec<String> = ClInfo::build_contents(&args.file_type)?;

        let extension = ClInfo::build_extension(&args.file_type)?;

        Ok(
            ClInfo { 
                args, 
                contents,
                extension,
            }
        ) 
    }

    pub fn get_arg(&self) -> &str {
        &self.args.file_type
    }

    pub fn get_contents(&self) -> &Vec<String> {
        &self.contents
    }

    pub fn get_extension(&self) -> &str {
        &self.extension
    }
}

fn write_basic_file(info: &ClInfo) -> Result<(), io::Error> {
    let file_name = "main".to_owned() + info.get_extension();

    println!("{file_name}");

    let contents = info.get_contents();

    let mut new_conts = String::new();

    for line in contents {
        new_conts.push_str(line);
        new_conts.push_str("\n");
    }

    fs::write(&file_name, new_conts)?;


    Ok(())
}

pub fn run(info: &ClInfo) -> Result<(), io::Error> {
   // write to the basic file 
    write_basic_file(&info)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // test if contents get correct data from the file for rust
    #[test]
    fn content_test_rust() {
        let file_type = String::from("rust");

        let result = vec!["fn main() {", "    println!(\"Test\\n\");", "}"];

        let contents = ClInfo::build_contents(&file_type).unwrap();

        assert_eq!(contents, result);
    }

    // same as above but for c
    #[test]
    fn content_test_c() {
        let file_type = String::from("c");

        let result = vec!["#include <stdio.h>",
                                             "", 
                                             "int main(void) {",
                                             "    printf(\"Test\\n\");",
                                             "",
                                             "    return 0;",
                                             "}"];

        let contents = ClInfo::build_contents(&file_type).unwrap();

        assert_eq!(contents, result);
    }

}
