use std::{io, rc::Rc};

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleNode {
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
    layers: Vec<Vec<MerkleNode>>,
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
            layers: Vec::new(),
        };

        tree.leaves = leaves;
        tree.build();

        tree
    }

    fn get_tree_height(&self) -> usize {
        let count = (2 * self.leaves.len() - 1) as f32;
        count.log2().floor() as usize
    }

    /// Builds the Merkle tree from a list of leaves. In case of an odd number of leaves, the last
    /// leaf is duplicated.
    fn build(&mut self) {
        let layer_count = self.get_tree_height();
        let mut layers = vec![self.leaves.to_vec()];

        for layer_index in 1..layer_count + 1 {
            let mut layer = Vec::new();
            let previous_layer = &layers[layer_index - 1];

            for i in (0..previous_layer.len()).step_by(2) {
                let left = &previous_layer[i];
                let right = if i + 1 < previous_layer.len() {
                    &previous_layer[i + 1]
                } else {
                    left
                };

                let parent = Self::create_parent(left, right);
                layer.push(parent);
            }
            layers.push(layer);
        }

        layers.reverse();
        self.layers = layers.clone();
        self.root = layers[0][0].clone();
    }

    /// Creates the parent node from the children, and updates their parent field.
    fn create_parent(left: &MerkleNode, right: &MerkleNode) -> MerkleNode {
        let mut data = String::new();
        data.push_str(&left.hash);
        data.push_str(&right.hash);

        let hash = <Sha256 as Digest>::digest(data);
        let res = hex::encode(hash);

        let parent = MerkleNode::new(res.clone());

        parent
    }

    pub fn generate_proof(&self, item: String) -> io::Result<Vec<MerkleNode>> {
        let mut proof = Vec::new();
        let Some(leaf_index) = self.leaves.iter().position(|e| e.hash == item) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "does not match any leaves",
            ));
        };

        let mut current_index = Self::get_tree_nodes(self.layers.len() - 1) + leaf_index;

        for layer_index in (1..self.layers.len()).rev() {
            let layer = &self.layers[layer_index];
            let internal_index = current_index - Self::get_tree_nodes(layer_index);
            let sibling = if internal_index % 2 == 0 {
                layer[internal_index + 1].clone()
            } else {
                layer[internal_index - 1].clone()
            };
            proof.push(sibling);

            current_index = (current_index - 1) / 2;
        }

        Ok(proof)
    }

    fn get_tree_nodes(height: usize) -> usize {
        (2usize).pow((height) as u32) - 1
    }

    pub fn verify_proof(&self, proof: Vec<MerkleNode>, leaf: String) -> bool {
        let mut current = MerkleNode::new(leaf);

        for elem in proof {
            current = Self::create_parent(&current, &elem);
        }

        self.root.hash == current.hash
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

        let hash = <Sha256 as Digest>::digest("0");
        let res = hex::encode(hash);

        let proof = tree.generate_proof(res.clone()).unwrap();

        assert!(tree.verify_proof(proof, res));

        let hash = <Sha256 as Digest>::digest("10");
        let res = hex::encode(hash);

        assert!(tree.generate_proof(res.clone()).is_err());
    }
}
