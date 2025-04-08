use std::sync::Arc;

use parking_lot::RwLock;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn arc_rwlock_serde<T, S>(item: &Arc<RwLock<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    item.read().serialize(serializer)
}

pub fn arc_rwlock_deserialize<'de, T, D>(deserializer: D) -> Result<Arc<RwLock<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let value = T::deserialize(deserializer)?;
    Ok(Arc::new(RwLock::new(value)))
}
