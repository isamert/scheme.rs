use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;

#[derive(Debug)]
pub enum PortData {
    FileInput(File),
    FileOutput(File)
}

impl PartialEq for PortData {
    fn eq(&self, rhs: &Self) -> bool {
        // FIXME: what should I do here?
        true
    }
}

impl Clone for PortData {
    fn clone(&self) -> Self {
        match self {
            PortData::FileInput(file) => PortData::FileInput(file.try_clone().expect("Can't copy file")),
            PortData::FileOutput(file) => PortData::FileOutput(file.try_clone().expect("Can't copy file")),
        }
    }
}

impl PortData {
    pub fn new_file_input(path: &str) -> PortData {
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect(&format!("Can't open file: {}", path));

        PortData::FileInput(file)
    }


    pub fn new_file_output(path: &str) -> PortData {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .expect(&format!("Can't open file: {}", path));

        PortData::FileOutput(file)
    }

    pub fn read_line(&self) -> (usize, String) {
        match self {
            PortData::FileInput(f) => {
                let mut string = String::new();
                let size = BufReader::new(f)
                    .read_line(&mut string)
                    .expect("Can't read file");

                (size, string)
            },
            _ => panic!("Can't read from this type of port.")
        }
    }

    pub fn read_all_str(&self) -> (usize, String) {
        match self {
            PortData::FileInput(f) => {
                let mut string = String::new();
                let size = BufReader::new(f)
                    .read_to_string(&mut string)
                    .expect("Can't read file");

                (size, string)
            },
            _ => panic!("Can't read from this type of port.")
        }
    }
}
