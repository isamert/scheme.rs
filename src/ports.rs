use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufReader, BufWriter, Stdin, Stdout};

#[derive(Debug)]
pub enum PortData {
    TextualFileInput(String, BufReader<File>),
    TextualFileOutput(String, BufWriter<File>),
    BinaryFileInput(String, BufReader<File>),
    BinaryFileOutput(String, BufWriter<File>),
    StdInput(Stdin),
    StdOutput(Stdout),
    Closed
}

impl PartialEq for PortData {
    fn eq(&self, _rhs: &Self) -> bool {
        // FIXME: what should I do here?
        true
    }
}

impl Clone for PortData {
    fn clone(&self) -> Self {
        panic!("this should not happen.")
    }
}

#[macro_export]
macro_rules! port_read_str_fn(
    ($br: ident, $fn: ident) => {
        {
            let mut result = String::new();
            let size = $br.$fn(&mut result)
                .expect("Can't read file");
            (size, result)
        }
    };
);


#[macro_export]
macro_rules! port_read_chr(
    ($br: ident) => {
        {
            let mut chr = [0; 1];
            $br.read_exact(&mut chr)
                .expect("Can't read file");
            (1, chr[0] as char)
        }
    };
);

impl PortData {
    pub fn new_textual_file_input(path: &str) -> PortData {
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect(&format!("Can't open file: {}", path));

        PortData::TextualFileInput(path.to_string(), BufReader::new(file))
    }


    pub fn new_textual_file_output(path: &str) -> PortData {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .expect(&format!("Can't open file: {}", path));

        PortData::TextualFileOutput(path.to_string(), BufWriter::new(file))
    }

    pub fn new_binary_file_input(path: &str) -> PortData {
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect(&format!("Can't open file: {}", path));

        PortData::BinaryFileInput(path.to_string(), BufReader::new(file))
    }

    pub fn new_binary_file_output(path: &str) -> PortData {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .expect(&format!("Can't open file: {}", path));

        PortData::BinaryFileOutput(path.to_string(), BufWriter::new(file))
    }

    pub fn current_input() -> PortData {
        // TODO: current_input should be changable
        PortData::StdInput(io::stdin())
    }

    //
    // Read functions
    //
    pub fn read_line(&mut self) -> (usize, String) {
        match self {
            PortData::TextualFileInput(_, br) => port_read_str_fn!(br, read_line),
            PortData::StdInput(br) => port_read_str_fn!(br, read_line),
            _ => panic!("Can't read from this type of port.")
        }
    }

    pub fn read_char(&mut self) -> (usize, char) {
        // FIXME: this only reads 1 u8 and casts it to char
        match self {
            PortData::TextualFileInput(_, br) => port_read_chr!(br),
            PortData::StdInput(br) => port_read_chr!(br),
            _ => panic!("Can't read from this type of port.")
        }
    }

    pub fn read_all_str(&mut self) -> (usize, String) {
        match self {
            PortData::TextualFileInput(_, br) => port_read_str_fn!(br, read_to_string),
            PortData::StdInput(br) => port_read_str_fn!(br, read_to_string),
            _ => panic!("Can't read from this type of port.")
        }
    }

    pub fn read_u8(&mut self) -> (usize, u8) {
        match self {
            PortData::BinaryFileInput(_, br) => {
                let mut u8s = [0; 1];
                br.read_exact(&mut u8s)
                    .expect("Can't read file");

                (1, u8s[0])
            },
            _ => panic!("Can't read from this type of port.")
        }
    }

    pub fn read_all_u8(&mut self) -> (usize, Vec<u8>) {
        match self {
            PortData::BinaryFileInput(_, br) => {
                let mut u8s = vec![];
                let size = br.read_to_end(&mut u8s)
                    .expect("Can't read file");

                (size, u8s)
            },
            _ => panic!("Can't read from this type of port.")
        }
    }

    //
    // Checks
    //
    pub fn is_input(&self) -> bool {
        match self {
            PortData::TextualFileInput(_, _) => true,
            PortData::BinaryFileInput(_, _) => true,
            PortData::StdInput(_) => true,
            _ => false
        }
    }

    pub fn is_output(&self) -> bool {
        match self {
            PortData::TextualFileOutput(_, _) => true,
            PortData::BinaryFileOutput(_, _) => true,
            PortData::StdOutput(_) => true,
            _ => false
        }
    }

    pub fn is_textual(&self) -> bool {
        match self {
            PortData::TextualFileInput(_, _) => true,
            PortData::TextualFileOutput(_, _) => true,
            _ => false
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            PortData::BinaryFileInput(_, _) => true,
            PortData::BinaryFileOutput(_, _) => true,
            _ => false
        }
    }
}
