use crate::compressor::CodeMap;
use crate::errors::CustomError;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

pub struct FileFormat;

impl FileFormat {
    pub fn write_coded_file(
        path: PathBuf,
        header: CodeMap,
        data: Vec<u8>,
    ) -> Result<(), CustomError> {
        let coded_file = File::create(path)?;

        let mut writer = BufWriter::new(coded_file);

        // write header to file
        writer.write_all(&(header.len() as u32).to_le_bytes())?;

        for (&ch, code) in header.iter() {
            writer.write_all(&(ch as u32).to_le_bytes())?;
            writer.write_all(&(code.len() as u8).to_le_bytes())?;
            writer.write_all(code)?;
        }
        // write data to file
        writer.write_all(&data)?;

        writer.flush()?;

        Ok(())
    }

    pub fn read_coded_file(buf_reader: &mut BufReader<File>) -> Result<CodeMap, CustomError> {
        let mut buff = [0u8; 4];

        // decompress Tree
        buf_reader.read_exact(&mut buff)?;
        let header_len = u32::from_le_bytes(buff);

        let mut code_map = CodeMap::new();

        let mut code_len_buf = [0u8; 1];

        for _ in 0..header_len {
            buf_reader.read_exact(&mut buff)?;
            let ch = char::from_u32(u32::from_le_bytes(buff)).ok_or(CustomError::CharNotFound)?;

            buf_reader.read_exact(&mut code_len_buf)?;

            let mut code_buf = vec![0u8; code_len_buf[0] as usize];
            buf_reader.read_exact(&mut code_buf)?;

            code_map.insert(ch, code_buf);
        }

        Ok(code_map)
    }
}
