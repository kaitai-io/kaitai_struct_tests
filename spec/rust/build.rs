use std::{
    io::{
        self,
        prelude::*
    },
    fs,
    path::Path, ffi::OsStr
};

pub fn path_exists(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}

fn main() {
    let source_path = Path::new("../../compiled/rust");
    let destination_path = Path::new("./formats");

    if !path_exists(destination_path) {
        fs::create_dir(destination_path).unwrap_or_else(|e| {
            println!("Unable to create dir: {}", e.to_string());
        });
    }

    let except_files = vec![
        "custom_fx_no_args.rs",
        "my_custom_fx.rs",
        "custom_fx.rs",
    ];

    remove_existing(destination_path, except_files).unwrap_or_else(|e| {
        println!("Unable to remove existing files under test: {}", e.to_string());
    });

    copy_new(source_path, destination_path).unwrap_or_else(|e| {
        println!("Unable to copy new files under test: {}", e.to_string());
    });

    println!("cargo:rerun-if-changed={}", source_path.display().to_string());
}

fn remove_existing(destination_path: &Path, except_files: Vec<&str>) -> io::Result<()> {
    for entry in fs::read_dir(destination_path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_os_string();

        if except_files.iter().find(|&&x| *x == file_name).is_none() {
            println!("cleaning {}", path.display().to_string());
            fs::remove_file(path)?;
        } else {
            println!("leaving {}", path.display().to_string());
        }
    }
    
    Ok(())
}

fn copy_new(source_path: &Path, destination_path: &Path) -> io::Result<()> {
    let mut librs = fs::File::create(destination_path.join("mod.rs"))?;

    write!(librs, "#![allow(unused_parens)]\n")?;
    write!(librs, "#![allow(dead_code)]\n")?;

    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().unwrap_or(&OsStr::new("")) != "rs" {
            continue;
        }

        if let Some(file_name) = path.file_name() {
            fs::copy(path.clone(), destination_path.join(file_name))?;
            println!("copying {} to {}", path.as_path().display().to_string(),
                destination_path.join(file_name).as_path().display().to_string());

            write!(librs, "pub mod {};\n",
                   path.file_stem().unwrap().to_str().unwrap())?;
            println!("updating lib.rs with {}", path.file_stem().unwrap().to_str().unwrap());
        }
    }
    
    Ok(())
}
