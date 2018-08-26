use std::io::{self, ErrorKind, Read};
use std::str;

static UTF8_CHAR_WIDTH: [u8; 256] = [
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

fn utf8_char_width(b: u8) -> usize {
    UTF8_CHAR_WIDTH[b as usize] as usize
}

fn read_one_byte(reader: &mut Read) -> Option<io::Result<u8>> {
    let mut buf = [0];
    loop {
        return match reader.read(&mut buf) {
            Ok(0) => None,
            Ok(..) => Some(Ok(buf[0])),
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => Some(Err(e)),
        };
    }
}

/// An iterator over the `char`s of a reader.
/// (A modified version of deprecated struct from here:
/// https://doc.rust-lang.org/1.26.0/src/std/io/mod.rs.html#2005)
#[derive(Debug)]
pub struct Chars<'a, R: 'a> {
    inner: &'a mut R,
}

impl<'a, R: Read> Chars<'a, R> {
    pub fn new(inner: &'a mut R) -> Chars<R> {
        Chars {
            inner,
        }
    }
}

impl<'a, R: Read> Iterator for Chars<'a, R> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let first_byte = match read_one_byte(&mut self.inner)? {
            Ok(b) => b,
            Err(_) => return None,
        };
        let width = utf8_char_width(first_byte);
        if width == 1 { return Some(first_byte as char) }
        if width == 0 { return None }
        let mut buf = [first_byte, 0, 0, 0];
        {
            let mut start = 1;
            while start < width {
                match self.inner.read(&mut buf[start..width]) {
                    Ok(0) => return None,
                    Ok(n) => start += n,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(_) => return None,
                }
            }
        }
        match str::from_utf8(&buf[..width]).ok() {
            Some(s) => Some(s.chars().next().unwrap()),
            None => None,
        }
    }
}
