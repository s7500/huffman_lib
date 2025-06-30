# Huffman Compression Library

A Rust library implementing Huffman coding for lossless data compression and decompression.

## Overview

This library provides an efficient implementation of the Huffman coding algorithm, which is a popular method for lossless data compression. It builds optimal prefix codes based on character frequency analysis and can significantly reduce file sizes for text and other data with non-uniform character distributions.

## Features

- **Lossless Compression**: Complete data integrity preservation
- **Optimal Encoding**: Uses Huffman trees for optimal prefix codes
- **File I/O Support**: Direct file compression and decompression
- **Custom File Format**: Includes header with encoding information
- **Error Handling**: Comprehensive error types for robust operation
- **Memory Efficient**: Streams data for large file processing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
huffman_lib = "0.1.0"
```

## Usage

### Basic Compression and Decompression

```rust
use huffman_lib::{compress, decompress};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compress a file
    let input_file = PathBuf::from("input.txt");
    let compressed_file = PathBuf::from("compressed.huff");
    compress(input_file, compressed_file)?;

    // Decompress the file
    let decompressed_file = PathBuf::from("output.txt");
    decompress(PathBuf::from("compressed.huff"), decompressed_file)?;

    Ok(())
}
```

### Error Handling

The library provides custom error types for different failure scenarios:

```rust
use huffman_lib::{compress, errors::CustomError};
use std::path::PathBuf;

match compress(PathBuf::from("input.txt"), PathBuf::from("output.huff")) {
    Ok(()) => println!("Compression successful!"),
    Err(CustomError::FileNotFound(path)) => eprintln!("File not found: {}", path),
    Err(CustomError::CompressionFailed) => eprintln!("Compression failed"),
    Err(e) => eprintln!("Error: {}", e),
}
```

## How It Works

1. **Frequency Analysis**: The algorithm analyzes the input data to count character frequencies
2. **Tree Construction**: Builds a Huffman tree where more frequent characters get shorter codes
3. **Code Generation**: Creates optimal binary codes for each character
4. **Compression**: Encodes the data using the generated codes
5. **File Format**: Stores the code table and compressed data in a custom format

### File Format

The compressed file format includes:
- Header length (4 bytes)
- Character codes table (variable length)
- Original character count (4 bytes)
- Compressed data (variable length)

## API Reference

### Functions

#### `compress(input: PathBuf, output: PathBuf) -> Result<(), CustomError>`

Compresses a file using Huffman coding.

- `input`: Path to the input file
- `output`: Path where the compressed file will be saved
- Returns: `Ok(())` on success, `CustomError` on failure

#### `decompress(input: PathBuf, output: PathBuf) -> Result<(), CustomError>`

Decompresses a Huffman-encoded file.

- `input`: Path to the compressed file
- `output`: Path where the decompressed file will be saved
- Returns: `Ok(())` on success, `CustomError` on failure

### Error Types

```rust
pub enum CustomError {
    FileNotFound(String),
    CharNotFound,
    InvalidInput,
    CompressionFailed,
    DecompressionFailed,
}
```

## Performance Characteristics

- **Time Complexity**: O(n log n) for compression, O(n) for decompression
- **Space Complexity**: O(n) where n is the size of the input
- **Compression Ratio**: Varies based on data entropy; better for text with repeated patterns

## Examples

### Compressing Text Files

```rust
use huffman_lib::compress;
use std::path::PathBuf;

// Compress a text file
let result = compress(
    PathBuf::from("document.txt"),
    PathBuf::from("document.huff")
);

match result {
    Ok(()) => println!("File compressed successfully!"),
    Err(e) => eprintln!("Compression failed: {}", e),
}
```

### Batch Processing

```rust
use huffman_lib::{compress, decompress};
use std::path::PathBuf;
use std::fs;

fn compress_directory(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let compressed_path = path.with_extension("huff");
            compress(path, compressed_path)?;
        }
    }
    Ok(())
}
```

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive tests for:
- Tree construction algorithms
- Encoding and decoding operations
- File format handling
- Error conditions
- Round-trip compression/decompression

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Based on the Huffman coding algorithm developed by David A. Huffman in 1952
- Implements optimal prefix-free coding for data compression

---

**Note**: This library is designed for educational and practical use. For production applications requiring maximum compression ratios, consider using more advanced algorithms like LZ77/LZ78 or modern compression formats.

Designed with ❤️ and passion for Rust programming.
