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
    root: String,
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
            root: String::new(),
        }
    }

    pub fn build(leaves: &mut [MerkleNode]) -> String {
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

            parents.push(Self::create_parent(Some(&mut left[0]), right));

            i += 2;
        }

        Self::build(&mut parents)
    }

    pub fn create_parent(
        left: Option<&mut MerkleNode>,
        right: Option<&mut MerkleNode>,
    ) -> MerkleNode {
        let mut data = String::new();

        if let Some(ref left) = left {
            data.push_str(&left.hash);
        }
        if let Some(ref right) = right {
            data.push_str(&right.hash);
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
