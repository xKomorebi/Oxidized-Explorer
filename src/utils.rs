use std::{env, fs, path::PathBuf, io};
use std::fs::Metadata;

pub fn get_current_directory() -> io::Result<fs::ReadDir> {
    let ret = env::current_dir()?;
    if !ret.is_dir() {
        return Err(io::Error::new(io::ErrorKind::Other, "Provided path is not a directory"));
    }
    fs::read_dir(ret)
}

pub fn get_dir_from_file(path: &PathBuf) -> io::Result<fs::ReadDir> {
    fs::read_dir(path)
}

pub fn list_cmd(path: &String) {
    match fs::metadata(path) {
        Ok(md) => {
            if !md.is_dir() {
                println!("Specified path does not point to a directory.");
                return;
            }
            let files = fs::read_dir(path).unwrap();
            for (index, file) in files.enumerate() {
                println!("{}: {}", index, file.unwrap().file_name().into_string().unwrap());
            }
        },
        Err(_) => {},
    }
}

pub fn get_file_details(path: &PathBuf) -> io::Result<Metadata> {
    fs::metadata(path)
}
