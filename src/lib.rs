mod compressor;
mod errors;
mod file_format;
mod tree;

use compressor::Compressor;
use file_format::FileFormat;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::result::Result;
use tree::Tree;

fn compress(input: PathBuf, output: PathBuf) -> Result<(), errors::CustomError> {
    let file = File::open(input)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();

    buf_reader.read_to_string(&mut content)?;
    let freq_map = content.chars().fold(HashMap::new(), |mut acc, letter| {
        let counter = acc.entry(letter).or_insert(0 as u64);
        *counter += 1;
        acc
    });

    let tree = Tree::build_tree(freq_map);
    let code_map = Compressor::to_encode(tree);
    let compressed_data = Compressor::to_compress(&content, &code_map);

    FileFormat::write_coded_file(output, code_map, compressed_data)
}

fn decompress(input: PathBuf, output: PathBuf) -> Result<(), errors::CustomError> {
    let file = File::open(input)?;
    let mut buf_reader = BufReader::new(file);
    let code_map = FileFormat::read_coded_file(&mut buf_reader)?;

    // decompress data
    let mut content = Vec::new();
    buf_reader.read_to_end(&mut content)?;
    let data = Compressor::to_decompress(&content, &code_map);

    //write data to file
    fs::write(output, data.concat())?;

    Ok(())
}
