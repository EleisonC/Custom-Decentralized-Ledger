use std::{fs, io::{self, Write}};

pub fn display_file_content(file_path: &str) -> io::Result<()> {
    let content = fs::read(file_path)?;

    let hex_string = content.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    // Print the hex string to the console
    println!("File Content (Hex):\n{}", hex_string);

    Ok(())
}

pub fn save_to_file(filename: &str, data: &[u8]) -> Result<(), String> {
    let mut file = match fs::File::create(filename) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error creating file: {}", err)),
    };

    if let Err(err) = file.write_all(data) {
        return Err(format!("Error writing to file: {}", err));
    }

    Ok(())
}
