// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

// Reference: original source code for this utility from `frewsxcv/rust-chunked-transfer`: \
//   https://github.com/frewsxcv/rust-chunked-transfer/blob/master/src/decoder.rs

use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result as IoResult;

pub struct Decoder<R> {
    source: R,
    remaining_chunks_size: Option<usize>,
}

impl<R> Decoder<R>
where
    R: Read,
{
    pub fn new(source: R) -> Decoder<R> {
        Decoder {
            source,
            remaining_chunks_size: None,
        }
    }

    fn read_chunk_size(&mut self) -> IoResult<usize> {
        let mut chunk_size_bytes = Vec::new();
        let mut has_ext = false;

        loop {
            let byte = match self.source.by_ref().bytes().next() {
                Some(b) => b?,
                None => return Err(IoError::new(ErrorKind::InvalidInput, DecoderError)),
            };

            if byte == b'\r' {
                break;
            }

            if byte == b';' {
                has_ext = true;
                break;
            }

            chunk_size_bytes.push(byte);
        }

        if has_ext {
            loop {
                let byte = match self.source.by_ref().bytes().next() {
                    Some(b) => b?,
                    None => return Err(IoError::new(ErrorKind::InvalidInput, DecoderError)),
                };
                if byte == b'\r' {
                    break;
                }
            }
        }

        self.read_line_feed()?;

        let chunk_size = String::from_utf8(chunk_size_bytes)
            .ok()
            .and_then(|c| usize::from_str_radix(c.trim(), 16).ok())
            .ok_or(IoError::new(ErrorKind::InvalidInput, DecoderError))?;

        Ok(chunk_size)
    }

    fn read_carriage_return(&mut self) -> IoResult<()> {
        match self.source.by_ref().bytes().next() {
            Some(Ok(b'\r')) => Ok(()),
            _ => Err(IoError::new(ErrorKind::InvalidInput, DecoderError)),
        }
    }

    fn read_line_feed(&mut self) -> IoResult<()> {
        match self.source.by_ref().bytes().next() {
            Some(Ok(b'\n')) => Ok(()),
            _ => Err(IoError::new(ErrorKind::InvalidInput, DecoderError)),
        }
    }
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let remaining_chunks_size = match self.remaining_chunks_size {
            Some(c) => c,
            None => {
                let chunk_size = self.read_chunk_size()?;

                if chunk_size == 0 {
                    self.read_carriage_return()?;
                    self.read_line_feed()?;
                    return Ok(0);
                }

                chunk_size
            }
        };

        if buf.len() < remaining_chunks_size {
            let read = self.source.read(buf)?;
            self.remaining_chunks_size = Some(remaining_chunks_size - read);
            return Ok(read);
        }

        assert!(buf.len() >= remaining_chunks_size);

        let buf = &mut buf[..remaining_chunks_size];
        let read = self.source.read(buf)?;

        self.remaining_chunks_size = if read == remaining_chunks_size {
            self.read_carriage_return()?;
            self.read_line_feed()?;
            None
        } else {
            Some(remaining_chunks_size - read)
        };

        Ok(read)
    }
}

#[derive(Debug, Copy, Clone)]
struct DecoderError;

impl fmt::Display for DecoderError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(fmt, "Error while decoding chunks")
    }
}

impl Error for DecoderError {
    fn description(&self) -> &str {
        "Error while decoding chunks"
    }
}
