use shard_types::{ShardDescription, ShardInfo, ShardId};

#[test]
fn test_shard_description_new() {
    let desc = ShardDescription::new(vec![0.1, 0.2, 0.3], "mô tả test".into());
    assert_eq!(desc.embedding.len(), 3);
    assert_eq!(desc.summary, "mô tả test");
}

#[test]
fn test_shard_description_serialize_roundtrip() {
    let desc = ShardDescription::new(vec![1.0], "test".into());
    let json = serde_json::to_string(&desc).unwrap();
    let decoded: ShardDescription = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.embedding, vec![1.0]);
    assert_eq!(decoded.summary, "test");
}

#[test]
fn test_shard_id_serialize_deserialize() {
    let mut id: ShardId = [0u8; 64];
    id[0] = 0xAB;
    id[63] = 0xCD;

    let info = ShardInfo {
        name: "test_shard".into(),
        hash_id: id,
        timestamp: 1234567890,
        size: 1024,
        tags: vec!["tag1".into(), "tag2".into()],
        description: ShardDescription::new(vec![], "".into()),
    };

    let json = serde_json::to_string(&info).unwrap();
    let decoded: ShardInfo = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded.name, "test_shard");
    assert_eq!(decoded.hash_id, id);
    assert_eq!(decoded.timestamp, 1234567890);
    assert_eq!(decoded.size, 1024);
    assert_eq!(decoded.tags, vec!["tag1", "tag2"]);
}

#[test]
fn test_shard_id_deserialize_invalid_length() {
    let json = r#"{
        "name": "bad",
        "hash_id": "abcd",
        "timestamp": 0,
        "size": 0,
        "tags": [],
        "description": {"embedding": [], "summary": ""}
    }"#;
    let result: Result<ShardInfo, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_shard_id_deserialize_invalid_hex() {
    let invalid_hex = "z".repeat(128);
    let json = format!(r#"{{
        "name": "bad",
        "hash_id": "{}",
        "timestamp": 0,
        "size": 0,
        "tags": [],
        "description": {{"embedding": [], "summary": ""}}
    }}"#, invalid_hex);
    let result: Result<ShardInfo, _> = serde_json::from_str(&json);
    assert!(result.is_err());
}

#[test]
fn test_shard_info_all_fields() {
    let mut id: ShardId = [0u8; 64];
    id[0] = 0xFF;

    let info = ShardInfo {
        name: "full_test".into(),
        hash_id: id,
        timestamp: 9999999999,
        size: 1048576,
        tags: vec!["ai".into(), "data".into(), "shard".into()],
        description: ShardDescription::new(vec![0.1, 0.2], "desc".into()),
    };

    assert_eq!(info.name, "full_test");
    assert_eq!(info.timestamp, 9999999999);
    assert_eq!(info.size, 1048576);
    assert_eq!(info.tags.len(), 3);
    assert_eq!(info.description.embedding.len(), 2);
}
