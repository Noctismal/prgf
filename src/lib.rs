use std::{fs, io};
use std::error::Error;

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
    file_info: FileInfo,
}

/// Holds the extension and contents of the file being made
#[derive(Debug)]
struct FileInfo {
    extension: String,
    contents: String,
}

impl ClInfo {
    pub fn build() -> Result<ClInfo, Box<dyn Error>> {
        // get command line arguments
        let args = Args::build();
        
        let file_info = ClInfo::get_file_info(&args.file_type)?;

        Ok(
            ClInfo { 
                args, 
                file_info,
            }
        ) 
    }

    // Helper functions
    
    fn get_file_info(file_type: &String) -> Result<FileInfo, Box<dyn Error>> {
        // prgf_langs holds extensions and contents for differnt languages
        let file = fs::read_to_string("prgf_langs.txt")?;
       
        // the language we are seraching for
        let lang = "// ".to_owned() + file_type;

        // holds file information
        let mut extension = String::new();
        let mut contents = String::new();

        let mut temp_contents: Vec<String> = vec![];
        
        let mut flag = false;

        for line in file.lines() {
            if line.contains(&lang) {
                // use a helper function to get the extension out of the line
                extension = ClInfo::get_extension(line.to_string())?;
                
                flag = true;
                continue;
            }

            if flag {
                // make sure that we arent at the end of that langs area
                if line.contains("// ") && line != lang {
                    break;
                }
                // needs to be turned into a string after this for loop
                temp_contents.push(line.to_string());
            }
        }

        // merge temp_contents vec into a string
        for line in temp_contents {
            contents.push_str(&line);
            contents.push_str("\n");
        }

    Ok( FileInfo {
        extension,
        contents,
    })
    }


    //*
    //* takes a string from the langs.txt file and looks for the extension by searching for e= ext
    fn get_extension(ext_line: String) -> Result<String, &'static str> {
        
        // split by spaces
        let mut ext_itter = ext_line.split(" ");
    
        while let Some(line) = ext_itter.next() {
            if line.contains("e=") {
                break;
            }
        }

        // if this returns Some(_) we have an extension
        if let Some(extension) = ext_itter.next() {
            Ok(extension.to_string())
        } else {
            // if we get to this point we couldnt find the extension
            Err("Extension not found in langs.txt")
        }
    } 
                
    pub fn get_arg(&self) -> &str {
        &self.args.file_type
    }

}

fn write_basic_file(info: &ClInfo) -> Result<(), io::Error> {
    let file_name = "main".to_owned();
    println!("{file_name}");

 //   let contents = info.get_contents();

    let mut new_conts = String::new();

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

        let result = String::from("fn main() {\n    println!(\"Test\\n\");\n}\n//\n");
        
        let file_info = ClInfo::get_file_info(&file_type).unwrap();
        let contents = file_info.contents;

        assert_eq!(result, contents);
    }

    // same as above but for c
    #[test]
    fn content_test_c() {
        let file_type = String::from("c");

        let result = String::from("#include <stdio.h>\n\nint main(void) {\n    printf(\"Test\\n\");\n\n    return 0;\n}\n");
        
        let file_info = ClInfo::get_file_info(&file_type).unwrap();
        let contents = file_info.contents;

        assert_eq!(result, contents);
    }

    // test if the program is getting the extension correctly
    #[test]
    fn extension_test() {
        let result = String::from(".rs");

        let extension = ClInfo::get_extension("// rust e= .rs".to_string()).unwrap();

        assert_eq!(result, extension);
    }
}
