use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufReader, BufWriter, Stdin, Stdout};

use serr::{SErr, SResult};
use utils::chars::Chars;
use utils::{new_rc_ref_cell, RcRefCell};

#[derive(Debug, Clone)]
pub enum PortData {
    TextualFileInput(String, RcRefCell<BufReader<File>>),
    TextualFileOutput(String, RcRefCell<BufWriter<File>>),
    BinaryFileInput(String, RcRefCell<BufReader<File>>),
    BinaryFileOutput(String, RcRefCell<BufWriter<File>>),
    StdInput(RcRefCell<Stdin>),
    StdOutput(RcRefCell<Stdout>),
    Closed
}

impl PartialEq for PortData {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (PortData::TextualFileInput(s,r), PortData::TextualFileInput(rs,rr))
                | (PortData::BinaryFileInput(s,r), PortData::BinaryFileInput(rs, rr)) => {
                    s == rs && &*r as *const _ == &*rr as *const _
            }
            (PortData::TextualFileOutput(s,r), PortData::TextualFileOutput(rs,rr))
                | (PortData::BinaryFileOutput(s,r), PortData::BinaryFileOutput(rs, rr)) => {
                    s == rs && &*r as *const _ == &*rr as *const _
            },
            (PortData::StdInput(r), PortData::StdInput(rr)) => {
                    &*r as *const _ == &*rr as *const _
            },
            (PortData::StdOutput(r), PortData::StdOutput(rr)) => {
                    &*r as *const _ == &*rr as *const _
            },
            _ => false
        }
    }
}

#[macro_export]
macro_rules! port_read_str_fn(
    ($br: ident, $fn: ident) => {{
        let br = &mut *$br.borrow_mut();
        let mut result = String::new();
        let size = br.$fn(&mut result)?;
        Ok((size, result))
    }};
);

impl PortData {
    pub fn new_textual_file_input(path: &str) -> SResult<PortData> {
        let file = OpenOptions::new()
            .read(true)
            .open(path)?;

        Ok(PortData::TextualFileInput(path.to_string(), new_rc_ref_cell(BufReader::new(file))))
    }


    pub fn new_textual_file_output(path: &str) -> SResult<PortData> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)?;

        Ok(PortData::TextualFileOutput(path.to_string(), new_rc_ref_cell(BufWriter::new(file))))
    }

    pub fn new_binary_file_input(path: &str) -> SResult<PortData> {
        let file = OpenOptions::new()
            .read(true)
            .open(path)?;

        Ok(PortData::BinaryFileInput(path.to_string(), new_rc_ref_cell(BufReader::new(file))))
    }

    pub fn new_binary_file_output(path: &str) -> SResult<PortData> {
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)?;

        Ok(PortData::BinaryFileOutput(path.to_string(), new_rc_ref_cell(BufWriter::new(file))))
    }

    //
    // Read functions
    //
    pub fn read_line(&mut self) -> SResult<(usize, String)> {
        match self {
            PortData::TextualFileInput(_, br) => port_read_str_fn!(br, read_line),
            PortData::StdInput(br) => port_read_str_fn!(br, read_line),
            // FIXME: fix this and the functions below
            _x => bail!(WrongPort => "read-line", "TODO:PORT_NAME_HERE")
        }
    }

    pub fn read_all_str(&mut self) -> SResult<(usize, String)> {
        match self {
            PortData::TextualFileInput(_, br) => port_read_str_fn!(br, read_to_string),
            PortData::StdInput(br) => port_read_str_fn!(br, read_to_string),
            _x => bail!(WrongPort => "read-all-str", "TODO:PORT_NAME_HERE")
        }
    }

    pub fn read_char(&mut self) -> SResult<(usize, char)> {
        // FIXME: this only reads 1 u8 and casts it to char
        macro_rules! port_read_chr(
            ($br: ident) => {{
                let br = &mut *$br.borrow_mut();
                let mut chr = [0; 1];
                br.read_exact(&mut chr)?;
                Ok((1, chr[0] as char))
            }};
        );

        match self {
            PortData::TextualFileInput(_, br) => port_read_chr!(br),
            PortData::StdInput(br) => port_read_chr!(br),
            _x => bail!(WrongPort => "read-char", "TODO:PORT_NAME_HERE")
        }
    }

    pub fn read_u8(&mut self) -> SResult<(usize, u8)> {
        match self {
            PortData::BinaryFileInput(_, br) => {
                let br = &mut *br.borrow_mut();
                let mut u8s = [0; 1];
                br.read_exact(&mut u8s)?;

                Ok((1, u8s[0]))
            },
            _x => bail!(WrongPort => "read-u8", "TODO:PORT_NAME_HERE")
        }
    }

    pub fn read_all_u8(&mut self) -> SResult<(usize, Vec<u8>)> {
        match self {
            PortData::BinaryFileInput(_, br) => {
                let br = &mut *br.borrow_mut();
                let mut u8s = vec![];
                let size = br.read_to_end(&mut u8s)?;

                Ok((size, u8s))
            },
            _x => bail!(WrongPort => "read-all-u8", "TODO:PORT_NAME_HERE")
        }
    }

    pub fn with_chars<F, T>(&mut self, f: F) -> SResult<T>
    where F: FnOnce(&mut Iterator<Item=char>) -> SResult<T> {
        macro_rules! with_chars(
            ($br: ident) => {{
                let br = &mut *$br.borrow_mut();
                let mut chars = Chars::new(br);
                f(&mut chars)
            }};
        );

        match self {
            PortData::TextualFileInput(_, br) => with_chars!(br),
            PortData::StdInput(br) => with_chars!(br),
            _x => bail!(WrongPort => "chars", "TODO:PORT_NAME_HERE")
        }
    }
    //
    // Write functions
    //
    pub fn write_string(&mut self, string: &str) -> SResult<()> {
        macro_rules! write_string(
            ($br: ident) => {{
                let br = &mut *$br.borrow_mut();
                write!(br, "{}", string)?;
                br.flush()?;
            }};
        );

        match self {
            PortData::TextualFileOutput(_,br) => write_string!(br),
            PortData::StdOutput(br) => write_string!(br),
            _x => bail!(WrongPort => "write-string", "TODO:PORT_NAME_HERE")
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


pub fn current_input_port() -> PortData {
    // TODO: current_input should be changable
    PortData::StdInput(new_rc_ref_cell(io::stdin()))
}

pub fn current_output_port() -> PortData {
    // TODO: current_output should be changable
    PortData::StdOutput(new_rc_ref_cell(io::stdout()))
}

