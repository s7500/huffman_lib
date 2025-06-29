use crate::tree::Tree;
use std::collections::HashMap;

pub type CodeMap = HashMap<char, Vec<u8>>;
pub struct Compressor;

impl Compressor {
    pub fn to_encode(tree: Tree) -> CodeMap {
        let mut code_map = HashMap::new();

        let mut stack = vec![(tree, vec![])];
        while let Some((node, code)) = stack.pop() {
            match node {
                Tree::Leaf { element, .. } => {
                    code_map.insert(element, code);
                }
                Tree::Node { left, right, .. } => {
                    let mut left_code = code.clone();
                    left_code.push(0);
                    let mut right_code = code.clone();
                    right_code.push(1);
                    stack.push((*left, left_code));
                    stack.push((*right, right_code));
                }
            }
        }

        code_map
    }

    fn flip_code_map(code_map: &CodeMap) -> HashMap<Vec<u8>, char> {
        let mut flipped_map = HashMap::new();
        for (&key, value) in code_map {
            flipped_map.insert(value.clone(), key.clone());
        }

        flipped_map
    }

    pub fn to_compress(content: &String, code_map: &CodeMap) -> (Vec<u8>, u32) {
        let mut bytes = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;
        let mut char_count = 0;

        for ch in content.chars() {
            char_count += 1;

            for &bit in &code_map[&ch] {
                if bit == 1 {
                    current_byte |= 1 << (7 - bit_count);
                }

                bit_count += 1;
                if bit_count == 8 {
                    bytes.push(current_byte);
                    current_byte = 0u8;
                    bit_count = 0;
                }
            }
        }

        if bit_count > 0 {
            bytes.push(current_byte);
        }

        (bytes, char_count)
    }

    pub fn to_decompress(
        content: &Vec<u8>,
        code_map: &CodeMap,
        char_count_b: [u8; 4],
    ) -> Vec<String> {
        let mut candidate = Vec::new();
        let mut tokens = Vec::new();
        let char_count = u32::from_le_bytes(char_count_b) as usize;

        let flipped_map = Compressor::flip_code_map(code_map);

        for &byte in content.into_iter() {
            let curr_byte = byte;

            for i in 0..8 {
                if tokens.len() >= char_count {
                    break;
                }
                let bit = (curr_byte >> (7 - i)) & 1;
                candidate.push(bit);

                match flipped_map.get(&candidate) {
                    Some(&ch) => {
                        tokens.push(ch.to_string());
                        candidate.clear();
                    }
                    None => (),
                }
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Tree;

    fn create_test_tree() -> Tree {
        let mut freq_map = HashMap::new();
        freq_map.insert('a', 3);
        freq_map.insert('b', 2);
        Tree::build_tree(freq_map)
    }

    #[test]
    fn test_to_encode_single_leaf() {
        let tree = Tree::Leaf {
            element: 'a',
            weight: 5,
        };
        let code_map = Compressor::to_encode(tree);

        assert_eq!(code_map.len(), 1);
        assert!(code_map.contains_key(&'a'));
        assert_eq!(code_map[&'a'], vec![]);
    }

    #[test]
    fn test_to_encode_two_leaves() {
        let tree = create_test_tree();
        let code_map = Compressor::to_encode(tree);

        assert_eq!(code_map.len(), 2);
        assert!(code_map.contains_key(&'a'));
        assert!(code_map.contains_key(&'b'));

        // One should be [0] and the other [1]
        let mut codes: Vec<Vec<u8>> = code_map.values().cloned().collect();
        codes.sort();
        assert_eq!(codes, vec![vec![0], vec![1]]);
    }

    #[test]
    fn test_flip_code_map() {
        let mut code_map = HashMap::new();
        code_map.insert('a', vec![0, 1]);
        code_map.insert('b', vec![1, 0]);

        let flipped = Compressor::flip_code_map(&code_map);

        assert_eq!(flipped.len(), 2);
        assert_eq!(flipped[&vec![0, 1]], 'a');
        assert_eq!(flipped[&vec![1, 0]], 'b');
    }

    #[test]
    fn test_compress_simple() {
        let mut code_map = HashMap::new();
        code_map.insert('a', vec![0]);
        code_map.insert('b', vec![1]);

        let content = "ab".to_string();
        let (compressed, char_count) = Compressor::to_compress(&content, &code_map);

        // 'a' = 0, 'b' = 1, so "ab" = 01 = 01000000 in binary (left-aligned)
        assert_eq!(compressed, vec![0b01000000]);
        assert_eq!(char_count, 2u32);
    }

    #[test]
    fn test_compress_multiple_bytes() {
        let mut code_map = HashMap::new();
        code_map.insert('a', vec![0, 0]);
        code_map.insert('b', vec![1, 1]);

        let content = "aaaa".to_string();
        let (compressed, char_count) = Compressor::to_compress(&content, &code_map);

        // 'a' = 00, so "aaaa" = 00000000 = 0
        assert_eq!(compressed, vec![0b00000000]);
        assert_eq!(char_count, 4u32);
    }

    #[test]
    fn test_compress_decompress_roundtrip() {
        let mut code_map = HashMap::new();
        code_map.insert('a', vec![0]);
        code_map.insert('b', vec![1]);

        let original = "abab".to_string();
        let (compressed, char_count) = Compressor::to_compress(&original, &code_map);
        let decompressed =
            Compressor::to_decompress(&compressed, &code_map, char_count.to_le_bytes());

        assert_eq!(decompressed.join(""), original);
    }

    #[test]
    fn test_decompress_simple() {
        let mut code_map = HashMap::new();
        code_map.insert('a', vec![0]);
        code_map.insert('b', vec![1]);

        // 01000000 = 'a' then 'b' (with padding)
        let compressed = vec![0b01000000];
        let decompressed = Compressor::to_decompress(&compressed, &code_map, 2u32.to_le_bytes());

        // Should contain "a" and "b" but might have extra characters due to padding
        assert!(decompressed.len() >= 2);
        assert_eq!(decompressed[0], "a");
        assert_eq!(decompressed[1], "b");
    }
}
