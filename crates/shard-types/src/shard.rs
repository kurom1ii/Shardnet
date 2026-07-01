use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub type ShardId = [u8; 64];

fn serialize_shard_id<S: Serializer>(id: &ShardId, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&hex::encode(id))
}

fn deserialize_shard_id<'de, D: Deserializer<'de>>(d: D) -> Result<ShardId, D::Error> {
    let hex_str = String::deserialize(d)?;
    let bytes = hex::decode(&hex_str).map_err(serde::de::Error::custom)?;
    let mut id = [0u8; 64];
    if bytes.len() != 64 {
        return Err(serde::de::Error::custom("ShardId must be 64 bytes"));
    }
    id.copy_from_slice(&bytes);
    Ok(id)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardDescription {
    pub embedding: Vec<f32>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardInfo {
    pub name: String,
    #[serde(
        serialize_with = "serialize_shard_id",
        deserialize_with = "deserialize_shard_id"
    )]
    pub hash_id: ShardId,
    pub timestamp: u64,
    pub size: u64,
    pub tags: Vec<String>,
    pub description: ShardDescription,
}

impl ShardDescription {
    pub fn new(embedding: Vec<f32>, summary: String) -> Self {
        ShardDescription { embedding, summary }
    }
}
