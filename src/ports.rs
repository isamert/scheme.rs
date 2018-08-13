use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;

#[derive(Debug)]
pub enum PortData {
    TextualFileInput(String, BufReader<File>),
    TextualFileOutput(String, BufWriter<File>)
}

impl PartialEq for PortData {
    fn eq(&self, rhs: &Self) -> bool {
        // FIXME: what should I do here?
        true
    }
}

impl Clone for PortData {
    fn clone(&self) -> Self {
        /*
        match self {
            PortData::TextualFileInput(file) => PortData::TextualFileInput(file.try_clone().expect("Can't copy file")),
            PortData::TextualFileOutput(file) => PortData::TextualFileOutput(file.try_clone().expect("Can't copy file")),
        }
        */
        panic!("this should not happen.")
    }
}

impl PortData {
    pub fn new_textual_file_input(path: &str) -> PortData {
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect(&format!("Can't open file: {}", path));

        PortData::TextualFileInput(path.to_string(), BufReader::new(file))
    }


    pub fn new_file_output(path: &str) -> PortData {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .expect(&format!("Can't open file: {}", path));

        PortData::TextualFileOutput(path.to_string(), BufWriter::new(file))
    }

    pub fn read_line(&mut self) -> (usize, String) {
        match self {
            PortData::TextualFileInput(_, br) => {
                let mut string = String::new();
                let size = br.read_line(&mut string)
                    .expect("Can't read file");

                (size, string)
            },
            _ => panic!("Can't read from this type of port.")
        }
    }

    pub fn read_all_str(&mut self) -> (usize, String) {
        match self {
            PortData::TextualFileInput(_, br) => {
                let mut string = String::new();
                let size = br.read_to_string(&mut string)
                    .expect("Can't read file");

                (size, string)
            },
            _ => panic!("Can't read from this type of port.")
        }
    }
}
