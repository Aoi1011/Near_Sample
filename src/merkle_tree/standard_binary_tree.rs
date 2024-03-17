use std::rc::Rc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MerkleNode {
    hash: String,
    parent: Option<Box<MerkleNode>>,
    left_child: Option<Box<MerkleNode>>,
    right_child: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    /// Store the hash and the parent
    pub fn new(hash: String) -> Self {
        Self {
            hash,
            parent: None,
            left_child: None,
            right_child: None,
        }
    }
}

#[derive(Debug)]
pub struct MerkleTree {
    root: MerkleNode,
    leaves: Vec<MerkleNode>,
}

impl MerkleTree {
    /// Stores the leaves and the root hash of the tree
    pub fn new(values: Vec<String>) -> Self {
        let mut leaves = Vec::new();

        for value in values {
            let hash = <Sha256 as Digest>::digest(value);
            let res = hex::encode(hash);
            let node = MerkleNode::new(res);
            leaves.push(node);
        }

        let mut tree = Self {
            root: MerkleNode::new("".to_string()),
            leaves: Vec::new(),
        };

        tree.build(&mut leaves);
        tree.leaves = leaves;

        tree
    }

    /// Builds the Merkle tree from a list of leaves. In case of an odd number of leaves, the last
    /// leaf is duplicated.
    fn build(&mut self, leaves: &mut [MerkleNode]) -> bool {
        let nleaves = leaves.len();
        if nleaves == 1 {
            if let Some(leaf) = leaves.get(0) {
                self.root = leaf.clone();
                return true;
            }
        }

        let mut parents = Vec::new();

        let mut i = 0;
        while i < nleaves {
            let (left, rest) = leaves.split_at_mut(i + 1);
            let right = if i + 1 < nleaves {
                rest.split_first_mut()
            } else {
                None
            };

            let right = match right {
                Some((right, _)) => Some(right),
                None => None,
            };

            let parent = Self::create_parent(Some(&mut left[i]), right);
            parents.push(parent);

            i += 2;
        }

        self.build(&mut parents)
    }

    /// Creates the parent node from the children, and updates their parent field.
    fn create_parent(left: Option<&mut MerkleNode>, right: Option<&mut MerkleNode>) -> MerkleNode {
        let mut data = String::new();

        match (&left, &right) {
            (Some(left), Some(right)) => {
                data.push_str(&left.hash);
                data.push_str(&right.hash);
            }
            (Some(left), None) => {
                data.push_str(&left.hash);
                data.push_str(&left.hash);
            }
            _ => unreachable!(),
        }

        let hash = <Sha256 as Digest>::digest(data);
        let res = hex::encode(hash);

        let mut parent = MerkleNode::new(res.clone());
        if let Some(left) = left {
            left.parent = Some(Box::new(parent.clone()));
            parent.left_child = Some(Box::new(left.clone()));
        }
        if let Some(right) = right {
            right.parent = Some(Box::new(parent.clone()));
            parent.right_child = Some(Box::new(right.clone()));
        }

        parent
    }

    pub fn get_audit_trail(&self, chunk_hash: String) -> Vec<(String, bool)> {
        for leaf in &self.leaves {
            if leaf.hash == chunk_hash {
                println!("Leaf exsits");
                return self.generate_audit_trail(&Rc::new(leaf.clone()), &mut vec![]);
            }
        }

        Vec::new()
    }

    /// Generates the audit trail in a bottom-up fasion
    fn generate_audit_trail(
        &self,
        merkle_node: &MerkleNode,
        trail: &mut Vec<(String, bool)>,
    ) -> Vec<(String, bool)> {
        let mut count = 0;
        count += 1;

        println!("Merkle node: {merkle_node:?}");

        if merkle_node == &self.root {
            trail.push((merkle_node.hash.clone(), true));
            return trail.to_vec();
        }

        let is_left = if let Some(ref left_child) = merkle_node.parent {
            left_child.as_ref() == merkle_node
        } else {
            false
        };

        if is_left {
            if let Some(right_child) = &merkle_node.parent {
                trail.push((right_child.hash.clone(), false));
                return self.generate_audit_trail(&merkle_node.parent.clone().unwrap(), trail);
            }
        } else {
            if let Some(left_child) = &merkle_node.parent {
                trail.push((left_child.hash.clone(), true));
                return self.generate_audit_trail(&merkle_node.parent.clone().unwrap(), trail);
            }
        }

        println!("Count: {count}");

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_merkletree() -> MerkleTree {
        let chunks = vec!["0", "1", "2", "3", "4", "5", "6", "7"]
            .iter_mut()
            .map(|chunk| chunk.to_string())
            .collect();

        MerkleTree::new(chunks)
    }

    #[test]
    fn test_build_merkletree() {
        let tree = construct_merkletree();

        assert_eq!(
            tree.root.hash,
            "e11a20bae8379fdc0ed560561ba33f30c877e0e95051aed5acebcb9806f6521f"
        );
    }

    #[test]
    fn test_get_audit_trail() {
        let tree = construct_merkletree();

        let hash = <Sha256 as Digest>::digest("2");
        let res = hex::encode(hash);

        let audit_trail = tree.get_audit_trail(res);

        println!("Audit trail: {:?}", audit_trail);

        // assert_eq!(
        //     tree.root.hash,
        //     "e11a20bae8379fdc0ed560561ba33f30c877e0e95051aed5acebcb9806f6521f"
        // );
    }
}
