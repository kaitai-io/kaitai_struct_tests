use std::{
    io::{
        self,
        prelude::*
    },
    fs,
    path::Path
};

fn main() {
    let source_path = Path::new("../../compiled/rust");
    let destination_path = Path::new("./src");
    
    remove_existing(destination_path).unwrap_or_else(|e| {
        println!("Unable to remove existing files under test: {}", e.to_string());
    });

    copy_new(source_path, destination_path).unwrap_or_else(|e| {
        println!("Unable to copy new files under test: {}", e.to_string());
    });
}

fn remove_existing(destination_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(destination_path)? {
        let entry = entry?;
        let path = entry.path();
        
        fs::remove_file(path)?;
    }
    
    Ok(())
}

fn copy_new(source_path: &Path, destination_path: &Path) -> io::Result<()> {
    let mut librs = fs::File::create(source_path.join("lib.rs"))?;

    write!(librs, "extern crate kaitai_struct;")?;
    
    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            fs::copy(path.clone(), destination_path.join(file_name))?;

            write!(librs, "pub mod {};\n",
                   path.file_stem().unwrap().to_str().unwrap())?;
        }
    }
    
    Ok(())
}
