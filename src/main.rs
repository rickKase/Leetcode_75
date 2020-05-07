use std::fs::File;
use std::io::{self, Result};
use std::path::Path;

use questions::q1;

mod questions;

fn main() {
    //prep();
}

fn prep() -> io::Result<()> {
    let n = 75;
    let pattern = "./src/questions/q{}.rs";
    
    create_files(n, pattern)?;

    println!("All files created successfully.");
    Ok(())
}

/*
Function: create_files

Description:
-------------
Creates `n` files following a given naming pattern.
The naming pattern should include a `{}` placeholder where the file index will be inserted.
For example, a pattern of "file_{}.txt" with n = 3 creates:
    - file_0.txt
    - file_1.txt
    - file_2.txt

Parameters:
- n: The number of files to create.
- pattern: A string slice representing the naming pattern.

Returns:
- An io::Result<()> indicating success or any I/O error encountered.
*/
fn create_files(n: usize, pattern: &str) -> Result<()> {
    for i in 0..n {
        let file_name = pattern.replace("{}", &i.to_string());
        
        // Check if the file already exists.
        let path = Path::new(&file_name);
        if path.exists() {
            println!("File '{}' already exists. Skipping...", file_name);
            continue;
        }
        
        // Create the file; this will overwrite an existing file with the same name.
        File::create(&file_name)?;
        println!("Created file: {}", file_name);
    }
    Ok(())
}
