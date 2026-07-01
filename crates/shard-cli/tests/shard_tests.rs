use shard_cli::{Shard, ShardContent};
use shard_types::ShardDescription;

#[test]
fn test_shard_content_size() {
    let content = ShardContent {
        value_content: vec![1, 2, 3, 4, 5],
    };
    assert_eq!(content.size(), 5);
}

#[test]
fn test_shard_content_size_empty() {
    let content = ShardContent {
        value_content: vec![],
    };
    assert_eq!(content.size(), 0);
}

#[test]
fn test_shard_content_hash_consistent() {
    let content1 = ShardContent {
        value_content: b"hello world".to_vec(),
    };
    let content2 = ShardContent {
        value_content: b"hello world".to_vec(),
    };
    assert_eq!(content1.hash(), content2.hash());
}

#[test]
fn test_shard_content_hash_different() {
    let content1 = ShardContent {
        value_content: b"hello world".to_vec(),
    };
    let content2 = ShardContent {
        value_content: b"hello worlD".to_vec(),
    };
    assert_ne!(content1.hash(), content2.hash());
}

#[test]
fn test_shard_content_hash_is_64_bytes() {
    let content = ShardContent {
        value_content: b"test".to_vec(),
    };
    let hash = content.hash();
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_shard_new_fields() {
    let desc = ShardDescription::new(vec![0.5], "test shard".into());
    let shard = Shard::new("my_shard".into(), b"content".to_vec(), desc);

    assert_eq!(shard.info.name, "my_shard");
    assert_eq!(shard.info.size, 7);
    assert!(shard.info.timestamp > 0);
    assert_eq!(shard.info.description.summary, "test shard");
    assert_eq!(shard.info.description.embedding, vec![0.5]);
    assert!(shard.info.tags.is_empty());
}

#[test]
fn test_shard_hash_matches_content_hash() {
    let data = b"unique data".to_vec();
    let desc = ShardDescription::new(vec![], "".into());
    let shard = Shard::new("test".into(), data.clone(), desc);

    let content_hash = ShardContent {
        value_content: data,
    }
    .hash();

    assert_eq!(shard.info.hash_id, content_hash);
}

#[test]
fn test_shard_serialize_roundtrip() {
    let desc = ShardDescription::new(vec![], "desc".into());
    let shard = Shard::new("test".into(), b"data".to_vec(), desc);

    let json = serde_json::to_string(&shard).unwrap();
    let decoded: Shard = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded.info.name, "test");
    assert_eq!(decoded.content.value_content, b"data".to_vec());
    assert_eq!(decoded.info.hash_id, shard.info.hash_id);
}

#[test]
fn test_shard_content_hash_deterministic() {
    let data = b"deterministic".to_vec();
    let hash1 = ShardContent {
        value_content: data.clone(),
    }
    .hash();
    let hash2 = ShardContent {
        value_content: data,
    }
    .hash();
    assert_eq!(hash1, hash2);
}

#[test]
fn test_shard_new_different_data_different_id() {
    let desc = ShardDescription::new(vec![], "".into());
    let shard1 = Shard::new("a".into(), b"data1".to_vec(), desc.clone());
    let shard2 = Shard::new("b".into(), b"data2".to_vec(), desc);

    assert_ne!(shard1.info.hash_id, shard2.info.hash_id);
    assert_eq!(shard1.info.size, 5);
    assert_eq!(shard2.info.size, 5);
}
