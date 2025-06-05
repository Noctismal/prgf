use std::{fs, io, path};
use std::error::Error;

use clap::Parser;
use dirs::config_dir;


/// Create a basic programming file
#[derive(Parser, Debug, Clone)]
#[command(version)]
struct Args {
    /// name of the language you wnat to make a file for
    #[arg(short, long)]
    file_type: String,

    #[arg(short, long)]
    project: Option<String>,
}

impl Args {
    // builds the arguments into the struct
    fn build() -> Args {
        Args::parse()
    }
}

/// Holds information regarding program execution
/// 
/// This includes the name of the language, the extension and the contents to be put
/// into the file
#[derive(Debug)]
pub struct ClInfo {
    /// Holds information gathered at the command line
    args: Args,
    /// Holds extension and contents to be put into the file being created
    file_info: FileInfo,
}

/// Holds the extension and contents of the file being made
#[derive(Debug)]
struct FileInfo {
    extension: String,
    contents: String,
}

impl ClInfo {
    /// Builds an instance of CLinfo with arguments from the command line
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

    /// Runs the logic of the program 
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        // see if the user wants a project or a basic file
        match self.args.project {
            Some(ref name) => &self.make_project(name.to_string())?,
            None => &self.write_basic_file()?,
        };

        Ok(())
    }

   // Helper functions
    
    fn get_file_info(file_type: &String) -> Result<FileInfo, Box<dyn Error>> {
        // get the users config directory
        let path = match config_dir() {
            Some(path) => path,
            None => return Err("Couldnt find config directory".into()),
        };

        let path = match path.to_str() {
            Some(path) => path,
            None => return Err("config directory couldnt be parsed as a string".into()),
        };

        // turn the path into a string and add the path to prgf_langs
        let mut config_dir: String = String::from(path);
        config_dir.push_str("/prgf/prgf_langs.txt");

        let file = fs::read_to_string(config_dir)?;
       
        // creates a string for the // <lang> we are looking for
        let lang = "// ".to_owned() + file_type;

        let mut extension = String::new();
        let mut contents = String::new();
        
        // gets file information, ending at the first instance of // after the one we are looking
        // for
        let mut flag = false;
        for line in file.lines() {
            // this gets the extension and sets the flag to true as everything past this we need
            if line.starts_with(&lang) && !flag {
                // use a helper function to get the extension out of the line
                extension = ClInfo::get_extension(line.to_string())?;
                
                flag = true;
                continue;
            }

            if flag {
                // make sure that we arent at the end of that langs area
                if line.starts_with("//") {
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
    

    /// Parses the line given to get the extension of the file
    fn get_extension(ext_line: String) -> Result<String, &'static str> {
        
        // split by spaces
        let mut ext_itter = ext_line.split(" ");

        let extension = match ext_itter.nth(2) {
            Some(ext) => ext,
            None => return Err("Extension not found in prgf_langs.txt"),
        };
    
        Ok(extension.to_string())
    } 

    /// Creates a file with the name main and the correct extension and writes the contents to it
    fn write_basic_file(&self) -> Result<(), io::Error> {
        let mut file_name = String::from("main");
        file_name.push_str(&self.file_info.extension);

        fs::write(&file_name, &self.file_info.contents)?;

        Ok(())
    }

    /// Creates a project directory with the name supplied or my_new_proj if none supplied
    fn make_project(&self, prg_name: String) -> Result<(), Box<dyn Error>> {
        // get the file path for where the file are 
        let file_path = path::PathBuf::from(prg_name).join("src");

        // this makes the project directory and the src directory
        fs::create_dir_all(&file_path)?;

        // make the file name
        let mut file_name = String::from("main");
        file_name.push_str(&self.file_info.extension);
        
        fs::write(file_path.join(file_name), &self.file_info.contents)?;

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

        let result = String::from("fn main() {\n    println!(\"Get programming\\n\");\n}\n");
        
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
