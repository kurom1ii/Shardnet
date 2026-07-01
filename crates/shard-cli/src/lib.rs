use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use shard_types::{ShardId, ShardDescription, ShardInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardContent {
    pub value_content: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shard {
    pub info: ShardInfo,
    pub content: ShardContent,
}

impl Shard {
    pub fn new(
        name: String,
        value_content: Vec<u8>,
        description: ShardDescription,
    ) -> Self {
        let content = ShardContent { value_content };
        let size = content.size();
        let hash_id = content.hash();
        let info = ShardInfo {
            name,
            hash_id,
            timestamp: current_timestamp(),
            size,
            tags: Vec::new(),
            description,
        };
        Shard { info, content }
    }
}

impl ShardContent {
    pub fn size(&self) -> u64 {
        self.value_content.len() as u64
    }

    pub fn hash(&self) -> ShardId {
        let mut hasher = Sha512::new();
        hasher.update(&self.value_content);
        let result = hasher.finalize();
        let mut id = [0u8; 64];
        id.copy_from_slice(&result);
        id
    }
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
