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

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        ClInfo::write_basic_file(&self)?;

        Ok(())
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
        
        let mut flag = false;

        for line in file.lines() {
            // this gets the extension and sets the flag to true as everything past this we need
            if line.contains(&lang) {
                // use a helper function to get the extension out of the line
                extension = ClInfo::get_extension(line.to_string())?;
                
                flag = true;
                continue;
            }

            if flag {
                // make sure that we arent at the end of that langs area
                if line.contains("// ") {
                    break;
                }
                contents.push_str(&line.to_string());
                contents.push('\n');
            }
        }


        Ok( FileInfo {
            extension,
            contents,
        })
    }
    

    /// takes a string from the langs.txt file and looks for the extension by searching for e= ext
    fn get_extension(ext_line: String) -> Result<String, &'static str> {
        
        // split by spaces
        let mut ext_itter = ext_line.split(" ");

        let extension = match ext_itter.nth(2) {
            Some(ext) => ext,
            None => return Err("Extension not found in prgf_langs.txt"),
        };
    
        Ok(extension.to_string())
    } 

    fn write_basic_file(&self) -> Result<(), io::Error> {
        let mut file_name = String::from("main");
        file_name.push_str(&self.file_info.extension);

        fs::write(&file_name, &self.file_info.contents)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test if contents get correct data from the file for rust
    #[test]
    fn content_test_rust() {
        let file_type = String::from("rust");

        let result = String::from("fn main() {\n    println!(\"Get programming\\n\");\n}\n//\n");
        
        let file_info = ClInfo::get_file_info(&file_type).unwrap();
        let contents = file_info.contents;

        assert_eq!(result, contents);
    }

    // same as above but for c
    #[test]
    fn content_test_c() {
        let file_type = String::from("c");

        let result = String::from("#include <stdio.h>\n\nint main(void) {\n    printf(\"Get programming\\n\");\n\n    return 0;\n}\n");
        
        let file_info = ClInfo::get_file_info(&file_type).unwrap();
        let contents = file_info.contents;

        assert_eq!(result, contents);
    }

    // test if the program is getting the extension correctly
    #[test]
    fn extension_test() {
        let result = String::from(".rs");

        let extension = ClInfo::get_extension("// rust .rs".to_string()).unwrap();

        assert_eq!(result, extension);
    }
}
