use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufReader, BufWriter, Stdin, Stdout};

use serr::{SErr, SResult};

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
            let size = $br.$fn(&mut result)?;
            Ok((size, result))
        }
    };
);


#[macro_export]
macro_rules! port_read_chr(
    ($br: ident) => {
        {
            let mut chr = [0; 1];
            $br.read_exact(&mut chr)?;
            Ok((1, chr[0] as char))
        }
    };
);

impl PortData {
    pub fn new_textual_file_input(path: &str) -> SResult<PortData> {
        let file = OpenOptions::new()
            .read(true)
            .open(path)?;

        Ok(PortData::TextualFileInput(path.to_string(), BufReader::new(file)))
    }


    pub fn new_textual_file_output(path: &str) -> SResult<PortData> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)?;

        Ok(PortData::TextualFileOutput(path.to_string(), BufWriter::new(file)))
    }

    pub fn new_binary_file_input(path: &str) -> SResult<PortData> {
        let file = OpenOptions::new()
            .read(true)
            .open(path)?;

        Ok(PortData::BinaryFileInput(path.to_string(), BufReader::new(file)))
    }

    pub fn new_binary_file_output(path: &str) -> SResult<PortData> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)?;

        Ok(PortData::BinaryFileOutput(path.to_string(), BufWriter::new(file)))
    }

    pub fn current_input() -> PortData {
        // TODO: current_input should be changable
        PortData::StdInput(io::stdin())
    }

    pub fn current_output() -> PortData {
        // TODO: current_output should be changable
        PortData::StdOutput(io::stdout())
    }

    //
    // Read functions
    //
    pub fn read_line(&mut self) -> SResult<(usize, String)> {
        match self {
            PortData::TextualFileInput(_, br) => port_read_str_fn!(br, read_line),
            PortData::StdInput(br) => port_read_str_fn!(br, read_line),
            // FIXME: fix this and the functions below
            x => bail!(WrongPort => "read-line".to_string(), "TODO:PORT_NAME_HERE")
        }
    }

    pub fn read_char(&mut self) -> SResult<(usize, char)> {
        // FIXME: this only reads 1 u8 and casts it to char
        match self {
            PortData::TextualFileInput(_, br) => port_read_chr!(br),
            PortData::StdInput(br) => port_read_chr!(br),
            x => bail!(WrongPort => "read-char".to_string(), "TODO:PORT_NAME_HERE")
        }
    }

    pub fn read_all_str(&mut self) -> SResult<(usize, String)> {
        match self {
            PortData::TextualFileInput(_, br) => port_read_str_fn!(br, read_to_string),
            PortData::StdInput(br) => port_read_str_fn!(br, read_to_string),
            x => bail!(WrongPort => "read-all-str".to_string(), "TODO:PORT_NAME_HERE")
        }
    }

    pub fn read_u8(&mut self) -> SResult<(usize, u8)> {
        match self {
            PortData::BinaryFileInput(_, br) => {
                let mut u8s = [0; 1];
                br.read_exact(&mut u8s)?;

                Ok((1, u8s[0]))
            },
            x => bail!(WrongPort => "read-u8".to_string(), "TODO:PORT_NAME_HERE")
        }
    }

    pub fn read_all_u8(&mut self) -> SResult<(usize, Vec<u8>)> {
        match self {
            PortData::BinaryFileInput(_, br) => {
                let mut u8s = vec![];
                let size = br.read_to_end(&mut u8s)?;

                Ok((size, u8s))
            },
            x => bail!(WrongPort => "read-all-u8".to_string(), "TODO:PORT_NAME_HERE")
        }
    }

    //
    // Write functions
    //
    pub fn write_string(&mut self, string: &str) -> SResult<()> {
        match self {
            PortData::TextualFileOutput(_,br) => {
                write!(br, "{}", string)?;
                br.flush()?;
            },
            PortData::StdOutput(br) => {
                write!(br, "{}", string)?;
                br.flush()?;
            },
            x => bail!(WrongPort => "write-string".to_string(), "TODO:PORT_NAME_HERE")
        };

        Ok(())
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
            PortData::StdOutput(_) => true,
            PortData::StdInput(_) => true,
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
