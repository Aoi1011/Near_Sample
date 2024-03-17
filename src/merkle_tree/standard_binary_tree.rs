use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
struct MerkleNode {
    hash: String,
    parent: Option<String>,
}

impl MerkleNode {
    pub fn new(hash: String) -> Self {
        Self { hash, parent: None }
    }
}

#[derive(Debug)]
pub struct MerkleTree {
    pub root: String,
}

impl MerkleTree {
    pub fn new(values: Vec<String>) -> Self {
        let mut leaves = Vec::new();

        for value in values {
            let hash = <Sha256 as Digest>::digest(value);
            let res = hex::encode(hash);
            let node = MerkleNode::new(res);
            leaves.push(node);
        }

        Self {
            root: Self::build(&mut leaves),
        }
    }

    /// Builds the Merkle tree from a list of leaves. In case of an odd number of leaves, the last
    /// leaf is duplicated.
    fn build(leaves: &mut [MerkleNode]) -> String {
        let nleaves = leaves.len();
        if nleaves == 1 {
            if let Some(leaf) = leaves.get(0) {
                return leaf.hash.clone();
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

            parents.push(Self::create_parent(Some(&mut left[i]), right));

            i += 2;
        }

        Self::build(&mut parents)
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

        if let Some(left) = left {
            left.parent = Some(res.clone());
        }
        if let Some(right) = right {
            right.parent = Some(res.clone());
        }

        let node = MerkleNode::new(res);

        node
    }
}

#[cfg(test)]
mod tests {
    use super::MerkleTree;

    #[test]
    fn test_merkletree() {
        let chunks = vec!["0", "1", "2", "3", "4", "5", "6", "7"]
            .iter_mut()
            .map(|chunk| chunk.to_string())
            .collect();

        let tree = MerkleTree::new(chunks);

        assert_eq!(
            tree.root,
            "e11a20bae8379fdc0ed560561ba33f30c877e0e95051aed5acebcb9806f6521f"
        );
    }
}
