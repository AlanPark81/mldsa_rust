pub struct MerkleTree<HashValueType>{
    tree: Vec<HashValueType>,
    hash_for_two: fn(&HashValueType, &HashValueType) -> HashValueType,
    input_length: usize
}

macro_rules! next_level_len {
    ($x:expr) => ($x-($x>>1))
}

/// Calculate and return the length of vector required to build a tree for the input vector length
macro_rules! tree_length_for_input {
    ($length:expr) => {{
        let mut len = $length;
        let mut count = 0;
        while len > 1 {
            count += len;
            len = next_level_len!(len);
        }
        count+1
    }}
}

macro_rules! create_next_level {
    ($level:expr, $hash_for_two:expr) => {{
        let length=$level.len();
        let mut i=0;
        let mut return_val=Vec::with_capacity(($level.len()+1)/2);
        while (i+1)<length{
            return_val.push($hash_for_two(&$level[i], &$level[i+1]));
            i+=2;
        }
        if i<length {
            return_val.push($hash_for_two(&$level[i], &$level[i]));
        }
        return_val
    }}
}

/// Construct a merkle tree of given the hash value vector and return the root value
#[macro_export]
macro_rules! calculate_merkle_root {
    ($hash_for_two:expr, $hashes:expr) => {{
        MerkleTree::create($hash_for_two, $hashes).root()
    }}
}


impl<HashValueType> MerkleTree<HashValueType> where HashValueType : Clone {
    /// Append a new hash value and build new merkle tree and return the root value
    pub fn append_hash(&mut self, new_hash:HashValueType) -> HashValueType {
        if self.input_length == 0 {
            self.tree=vec![new_hash];
        } else {
            let mut new_tree=Vec::with_capacity(tree_length_for_input!(self.input_length+1));
            let mut level = self.tree[0..self.input_length].to_vec();
            level.push(new_hash);
            let mut rest = self.tree[self.input_length..].to_vec();
            while level.len()>1 {
                let mut next_level={
                    if next_level_len!(level.len())>=rest.len() {
                        rest[..].to_vec()
                    } else {
                        rest[0..next_level_len!(level.len())].to_vec()
                    }
                };
                rest=rest[next_level.len()..].to_vec();
                let last=level[level.len()-1].clone();
                if level.len() % 2 == 1 {
                    next_level.push((self.hash_for_two)(&last, &last));
                } else if level.len()==2 {
                    next_level.push((self.hash_for_two)(&level[level.len()-2], &level[level.len()-1]));
                } else {
                    let last_index=next_level.len()-1;
                    next_level[last_index]=(self.hash_for_two)(&level[level.len()-2], &level[level.len()-1]);
                }
                new_tree.append(&mut level);
                level=next_level;
            }
            new_tree.append(&mut level);
            self.tree=new_tree;
        }
        self.input_length+=1;
        assert_eq!(tree_length_for_input!(self.input_length), self.tree.len());
        self.root()
    }

    /// Construct a tree that has only one element containing the hash value of empty byte vector
    pub fn empty_tree(hash_for_two:fn(&HashValueType, &HashValueType)->HashValueType) -> MerkleTree<HashValueType> {
        let tree=vec![];
        MerkleTree{ tree, hash_for_two:hash_for_two, input_length:0 }
    }

    /// Construct the merkle tree containing the input vector and their ancestors that are built with hash_for_two_hashes
    /// # Example
    ///
    /// ```
    /// use mldsa_rust::merkle_tree;
    /// fn hash_for_two_hashes(a:&Vec<u8>, b:&Vec<u8>) -> Vec<u8> {a.clone()}
    /// let mut input = Vec::new();
    /// let hash_value=vec![0u8];
    /// input.push(hash_value);
    ///
    /// let merkle_tree=merkle_tree::MerkleTree::<Vec<u8>>::create(hash_for_two_hashes,&input);
    /// ```
    pub fn create(hash: fn(&HashValueType, &HashValueType) -> HashValueType, hashes: &Vec<HashValueType>) -> MerkleTree<HashValueType>{
        let hashes_len=hashes.len();
        if hashes_len == 0 {
            return MerkleTree::empty_tree(hash)
        }

        let tree_size = tree_length_for_input!(hashes_len);
        let mut tree: Vec<HashValueType> = Vec::with_capacity(tree_size);

        for hash in hashes {
            tree.push(hash.clone());
        }

        let mut level:Vec<HashValueType>= hashes.to_vec();
        while level.len()>1 {
            level=create_next_level!(&level, hash);
            tree.append(&mut level.clone());
        }

        MerkleTree {
            tree,
            hash_for_two:hash,
            input_length:hashes_len
        }
    }

    /// Return the hash value stored in the root element of this tree
    pub fn root(&self) -> HashValueType {
        self.tree[self.tree.len()-1].clone()
    }

    /// Return the size of merkle tree
    pub fn size(&self) -> usize {
        self.tree.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blake2_rfc::blake2b::{blake2b, Blake2b};

    const HASH_LENGTH: usize=32;
    fn hash_two(left:&Vec<u8>, right:&Vec<u8>)->Vec<u8>{
        let mut blake=Blake2b::new(HASH_LENGTH);
        blake.update(&left);
        blake.update(&right);

        let mut return_val=vec![0u8;HASH_LENGTH];
        return_val.copy_from_slice(blake.finalize().as_bytes());
        return_val
    }

    #[test]
    fn it_calculates_length_for_a_size_two_input() {
        let size = 2;
        assert_eq!(tree_length_for_input!(size), 3);
    }

    #[test]
    fn it_calculates_length_for_a_size_three_input() {
        let size = 3;
        assert_eq!(tree_length_for_input!(size), 6);
    }

    #[test]
    fn it_makes_a_merkle_tree_from_a_single_hash() {
        let bytes = vec![0xAA; 32];
        let hash = blake2b(32, &[], &bytes);//hash(&bytes, 32);
        let expected_root = hash.as_bytes();
        assert_eq!(calculate_merkle_root!(hash_two, &vec![hash.as_bytes().to_vec()]), expected_root);
    }

    #[test]
    fn it_makes_a_empty_merkle_tree_and_append_a_single_hash() {
        let bytes = vec![0xAA; 32];
        let hash = blake2b(32, &[], &bytes);//hash(&bytes, 32);

        let mut merkle_tree = MerkleTree::create(hash_two,&Vec::new());
        merkle_tree.append_hash(hash.as_bytes().to_vec());
        let expected_root = hash.as_bytes();
        assert_eq!(merkle_tree.root(), expected_root);
    }

    #[test]
    fn it_makes_a_merkle_tree_from_two_hashes() {
        let bytes_1 = vec![0xAA; 32];
        let bytes_2 = vec![0xBB; 32];

        let hash_1 = blake2b(32, &[], &bytes_1);//hash(&bytes_1, 32);
        let hash_2 = blake2b(32, &[], &bytes_2);//= hash(&bytes_2, 32);

        let hashes = vec![hash_1.as_bytes().to_vec().clone(), hash_2.as_bytes().to_vec().clone()];
        let expected_root = hash_two(&hash_1.as_bytes().to_vec(), &hash_2.as_bytes().to_vec());

        assert_eq!(calculate_merkle_root!(hash_two, &hashes), expected_root);
    }

    #[test]
    fn it_makes_a_merkle_tree_from_one_hashes_and_append_a_hash() {
        let bytes_1 = vec![0xFF; 32];
        let bytes_2 = vec![0xAA; 32];

        let hash_1 = blake2b(32, &[], &bytes_1);//hash(&bytes_1, 32);
        let hash_2 = blake2b(32, &[], &bytes_2);//= hash(&bytes_2, 32);

        let hashes = vec![hash_1.as_bytes().to_vec().clone()];
        let expected_root = hash_two(&hash_1.as_bytes().to_vec(), &hash_2.as_bytes().to_vec());

        let mut merkle_tree = MerkleTree::create(hash_two, &hashes);
        merkle_tree.append_hash(hash_2.as_bytes().to_vec().clone());
        assert_eq!(merkle_tree.root(), expected_root);
    }

    #[test]
    fn it_makes_a_merkle_tree_from_three_hashes() {
        let bytes_1 = vec![0xAA; 32];
        let bytes_2 = vec![0xBB; 32];
        let bytes_3 = vec![0xCC; 32];

        let hash_1 = blake2b(32, &[], &bytes_1);//hash(&bytes_1, 32);
        let hash_2 = blake2b(32, &[], &bytes_2);//hash(&bytes_2, 32);
        let hash_3 = blake2b(32, &[], &bytes_3);//hash(&bytes_3, 32);

        let hashes = vec![hash_1.as_bytes().to_vec().clone(), hash_2.as_bytes().to_vec().clone(), hash_3.as_bytes().to_vec().clone()];

        let two_hash_1 = hash_two(&hash_1.as_bytes().to_vec(), &hash_2.as_bytes().to_vec());
        let two_hash_2 = hash_two(&hash_3.as_bytes().to_vec(), &hash_3.as_bytes().to_vec());

        let expected_root = hash_two(&two_hash_1, &two_hash_2);

        assert_eq!(calculate_merkle_root!(hash_two, &hashes), expected_root);
    }

    #[test]
    fn it_makes_a_merkle_tree_from_two_hashes_and_append_a_hash() {
        let bytes_1 = vec![0xAA; 32];
        let bytes_2 = vec![0xBB; 32];
        let bytes_3 = vec![0xCC; 32];

        let hash_1 = blake2b(32, &[], &bytes_1);//hash(&bytes_1, 32);
        let hash_2 = blake2b(32, &[], &bytes_2);//hash(&bytes_2, 32);
        let hash_3 = blake2b(32, &[], &bytes_3);//hash(&bytes_3, 32);

        let hashes = vec![hash_1.as_bytes().to_vec().clone(), hash_2.as_bytes().to_vec().clone()];

        let two_hash_1 = hash_two(&hash_1.as_bytes().to_vec(), &hash_2.as_bytes().to_vec());
        let two_hash_2 = hash_two(&hash_3.as_bytes().to_vec(), &hash_3.as_bytes().to_vec());

        let expected_root = hash_two(&two_hash_1, &two_hash_2);

        let mut merkle_tree = MerkleTree::create(hash_two, &hashes);
        merkle_tree.append_hash(hash_3.as_bytes().to_vec().clone());

        assert_eq!(merkle_tree.root(), expected_root);
    }
}
