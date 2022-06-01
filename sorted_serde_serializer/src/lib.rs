use itertools::Itertools;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasher,
    hash::Hash,
};

/// Trait that allows wrapping the initial type to a target newtype that has a custom
/// serializer that serializes with sorting
pub trait SerializeSortedFrom {
    type Target: Serialize;
    fn sorted_from(t: Self) -> Self::Target;
}

/// Wrapper to serialize a hashmap with sorted keys
pub struct SerializeSortedHashMapWrapper<'a, K, V, H>(&'a HashMap<K, V, H>);

impl<'a, K, V, H> Serialize for SerializeSortedHashMapWrapper<'a, K, V, H>
where
    K: std::cmp::Ord + Hash + Serialize,
    H: BuildHasher,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let h = self.0;
        let mut map = serializer.serialize_map(Some(h.len()))?;
        for key in h.keys().sorted() {
            map.serialize_entry(key, h.get(key).unwrap())?;
        }
        map.end()
    }
}

impl<'a, K, V, H> SerializeSortedFrom for &'a HashMap<K, V, H>
where
    K: std::cmp::Ord + Hash + Serialize,
    H: BuildHasher,
    V: Serialize,
{
    type Target = SerializeSortedHashMapWrapper<'a, K, V, H>;

    fn sorted_from(t: &'a HashMap<K, V, H>) -> Self::Target {
        SerializeSortedHashMapWrapper::<'a, K, V, H>(t)
    }
}

/// Wrapper to serialize a hashmap with sorted keys
impl<'a, K, V, H> SerializeSortedFrom for &'a Option<HashMap<K, V, H>>
where
    K: std::cmp::Ord + Hash + Serialize,
    H: BuildHasher,
    V: Serialize,
{
    type Target = Option<SerializeSortedHashMapWrapper<'a, K, V, H>>;

    fn sorted_from(t: &'a Option<HashMap<K, V, H>>) -> Self::Target {
        t.as_ref().map(SerializeSortedHashMapWrapper::<'a, K, V, H>)
    }
}

pub struct SerializeSortedHashSetWrapper<'a, K, H>(&'a HashSet<K, H>);

impl<'a, K, H> Serialize for SerializeSortedHashSetWrapper<'a, K, H>
where
    K: std::cmp::Ord + Hash + Serialize,
    H: BuildHasher,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let h = self.0;
        let mut map = serializer.serialize_seq(Some(h.len()))?;
        for key in h.iter().sorted() {
            map.serialize_element(key)?;
        }
        map.end()
    }
}

impl<'a, K, H> SerializeSortedFrom for &'a HashSet<K, H>
where
    K: std::cmp::Ord + Hash + Serialize,
    H: BuildHasher,
{
    type Target = SerializeSortedHashSetWrapper<'a, K, H>;

    fn sorted_from(t: &'a HashSet<K, H>) -> Self::Target {
        SerializeSortedHashSetWrapper::<'a, K, H>(t)
    }
}

/// Wrapper to serialize a hashmap with sorted keys
impl<'a, K, H> SerializeSortedFrom for &'a Option<HashSet<K, H>>
where
    K: std::cmp::Ord + Hash + Serialize,
    H: BuildHasher,
{
    type Target = Option<SerializeSortedHashSetWrapper<'a, K, H>>;

    fn sorted_from(t: &'a Option<HashSet<K, H>>) -> Self::Target {
        t.as_ref().map(SerializeSortedHashSetWrapper::<'a, K, H>)
    }
}

pub fn serialize<'a, S, V>(v: &'a V, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    &'a V: SerializeSortedFrom,
{
    SerializeSortedFrom::sorted_from(v).serialize(serializer)
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
