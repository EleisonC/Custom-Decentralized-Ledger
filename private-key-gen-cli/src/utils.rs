use std::{fs, io::{self, Read}};

pub fn display_file_content(file_path: &str) -> io::Result<()> {
    let mut content = fs::read(file_path)?;

    let hex_string = content.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    // Print the hex string to the console
    println!("File Content (Hex):\n{}", hex_string);

    Ok(())
}