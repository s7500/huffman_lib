use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
