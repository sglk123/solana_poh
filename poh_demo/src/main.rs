use std::time::{Duration, SystemTime};
use sha2::{Sha256, Digest};

// basic poh struct
struct ProofOfHistory {
    current_slot: u64,
    current_timestamp: SystemTime,
}

impl ProofOfHistory {
    // init
    fn new() -> Self {
        ProofOfHistory {
            current_slot: 0,
            current_timestamp: SystemTime::now(),
        }
    }

    // 更新当前时间戳、槽位和哈希序列
    fn update(&mut self) {
        self.current_slot += 1;
        self.current_timestamp += Duration::from_secs(1);
    }

    fn get_current_slot(&self) -> u64 {
        self.current_slot
    }

    fn get_current_timestamp(&self) -> SystemTime {
        self.current_timestamp
    }

    fn calculate_hash(&self) -> Vec<u8> {
        let input = format!("{}-{:?}", self.current_slot, self.current_timestamp);
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }
}

struct PoH {
    hashes: Vec<Vec<u8>>,
}

impl PoH {
    fn new() -> Self {
        PoH {
            hashes: Vec::new(),
        }
    }

    fn add_hash(&mut self, hash: Vec<u8>) {
        self.hashes.push(hash);
    }

    fn get_last_hash(&self) -> Option<&Vec<u8>> {
        self.hashes.last()
    }

    // generate by proof_of_history hash and last hash
    fn generate_next_hash(&mut self, mut hash: Vec<u8>) {
        if let Some(last_hash) = self.get_last_hash() {
            let mut hasher = Sha256::new();
            hash.extend(last_hash);
            hasher.update(hash);
            let new_hash = hasher.finalize().to_vec();
            self.add_hash(new_hash);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_of_history() {
        let mut poh = ProofOfHistory::new();

        assert_eq!(poh.get_current_slot(), 0);

        let initial_timestamp = poh.get_current_timestamp();

        poh.update();
        assert_eq!(poh.get_current_slot(), 1);
        assert!(poh.get_current_timestamp() > initial_timestamp);
        assert_eq!(poh.get_hash_sequence().len(), 1);

        poh.update();
        assert_eq!(poh.get_current_slot(), 2);
        assert!(poh.get_current_timestamp() > initial_timestamp);
        assert_eq!(poh.get_hash_sequence().len(), 2);
    }
}

fn main() {
    // init
    let mut poh = ProofOfHistory::new();
    println!("Initial slot: {}", poh.get_current_slot());
    println!("Initial timestamp: {:?}", poh.get_current_timestamp());

    // init hash sequence
    let hash = poh.calculate_hash();
    let mut poh_hash = PoH::new();
    poh_hash.add_hash(hash);

    // update
    poh.update();
    println!("Updated slot: {}", poh.get_current_slot());
    println!("Updated timestamp: {:?}", poh.get_current_timestamp());
    let hash = poh.calculate_hash();
    poh_hash.generate_next_hash(hash);

    for hash in &poh_hash.hashes {
        println!("{:?}", hash);
    }
}