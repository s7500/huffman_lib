use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tree {
    Leaf {
        element: char,
        weight: u64,
    },
    Node {
        weight: u64,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Tree {
    fn weight(&self) -> u64 {
        match self {
            Self::Leaf { element: _, weight } => *weight,
            Self::Node { weight, .. } => *weight,
        }
    }

    pub fn build_tree(data: HashMap<char, u64>) -> Tree {
        let mut heap: BinaryHeap<Reverse<Tree>> = BinaryHeap::new();
        for (k, v) in data {
            let leaf = Tree::Leaf {
                element: k,
                weight: v,
            };
            heap.push(Reverse(leaf));
        }

        while heap.len() > 1 {
            let Reverse(node1) = heap.pop().unwrap();
            let Reverse(node2) = heap.pop().unwrap();

            let internal = Tree::Node {
                weight: node1.weight() + node2.weight(),
                left: Box::new(node1),
                right: Box::new(node2),
            };
            heap.push(Reverse(internal))
        }

        heap.pop().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_weight() {
        let leaf = Tree::Leaf {
            element: 'a',
            weight: 5,
        };
        assert_eq!(leaf.weight(), 5);
    }

    #[test]
    fn test_node_weight() {
        let left = Tree::Leaf {
            element: 'a',
            weight: 3,
        };
        let right = Tree::Leaf {
            element: 'b',
            weight: 7,
        };
        let node = Tree::Node {
            weight: 10,
            left: Box::new(left),
            right: Box::new(right),
        };
        assert_eq!(node.weight(), 10);
    }

    #[test]
    fn test_build_tree_single_character() {
        let mut freq_map = HashMap::new();
        freq_map.insert('a', 5);

        let tree = Tree::build_tree(freq_map);
        match tree {
            Tree::Leaf { element, weight } => {
                assert_eq!(element, 'a');
                assert_eq!(weight, 5);
            }
            _ => panic!("Expected a leaf node for single character"),
        }
    }

    #[test]
    fn test_build_tree_two_characters() {
        let mut freq_map = HashMap::new();
        freq_map.insert('a', 3);
        freq_map.insert('b', 7);

        let tree = Tree::build_tree(freq_map);
        match tree {
            Tree::Node { weight, .. } => {
                assert_eq!(weight, 10);
            }
            _ => panic!("Expected a node for two characters"),
        }
    }

    #[test]
    fn test_build_tree_multiple_characters() {
        let mut freq_map = HashMap::new();
        freq_map.insert('a', 5);
        freq_map.insert('b', 9);
        freq_map.insert('c', 12);
        freq_map.insert('d', 13);
        freq_map.insert('e', 16);
        freq_map.insert('f', 45);

        let tree = Tree::build_tree(freq_map);
        assert_eq!(tree.weight(), 100);
    }

    #[test]
    fn test_tree_ordering() {
        let tree1 = Tree::Leaf {
            element: 'a',
            weight: 5,
        };
        let tree2 = Tree::Leaf {
            element: 'b',
            weight: 10,
        };

        assert!(tree1 < tree2);
        assert!(Reverse(tree1) > Reverse(tree2));
    }
}
