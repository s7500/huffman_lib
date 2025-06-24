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

    pub fn to_compress(content: &String, code_map: &CodeMap) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_count = 0;

        for ch in content.chars() {
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

        bytes
    }

    pub fn to_decompress(content: &Vec<u8>, code_map: &CodeMap) -> Vec<String> {
        let mut candidate = Vec::new();
        let mut tokens = Vec::new();

        let flipped_map = Compressor::flip_code_map(code_map);

        for &byte in content.into_iter() {
            let curr_byte = byte;
            for i in 0..8 {
                let bit = (curr_byte >> (7 - i)) & 1;
                candidate.push(bit);

                match flipped_map.get(&candidate) {
                    Some(&ch) => {
                        tokens.push(ch.to_string());
                        candidate.clear();
                    }
                    None => {}
                }
            }
        }

        tokens
    }
}
