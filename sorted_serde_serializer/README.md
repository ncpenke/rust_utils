# sorted_serde_serializer

Serialize HashSet and HashMap with sorted keys. This is useful for persisting to JSON or other text format with consistent ordering, since the built-in HashMap and HashSet don't have consistent ordering due to secure hashing.

## Example

```rust
use std::collections::{HashMap, HashSet};
use serde::Serialize;

#[derive(Serialize)]
struct TestStruct {
    #[serde(serialize_with = "sorted_serde_serializer::serialize")]
    hash_map: HashMap<String, String>,
    #[serde(serialize_with = "sorted_serde_serializer::serialize")]
    optional_hash_map: HashMap<String, String>,
    #[serde(serialize_with = "sorted_serde_serializer::serialize")]
    hash_set: HashSet<String>,
    #[serde(serialize_with = "sorted_serde_serializer::serialize")]
    optional_hash_set: HashSet<String>,
}
```
