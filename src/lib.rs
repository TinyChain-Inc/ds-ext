//! This crate repackages standard data structures with additional capabilities,
//! like fast ordered maps and sets.
//!
//! [`OrdHashSet`] uses a [`Vec`] internally for ordering.
//! [`OrdHashSet`] and [`OrdHashMap`] both implement a `bisect` method
//! which allows looking up a key by comparison,
//! potentially avoiding the need for a heap allocation to construct a search key..
//!
//! Features:
//!  - `all`: enables all features
//!  - `serialize`: enables support for [`serde`](https://docs.rs/serde/).
//!  - `stream`: enables support for [`destream`](https://docs.rs/destream/).
//!  - `hash`: enables support for [`async-hash`](https://docs.rs/async-hash/).
//!
//!
//! Example usage:
//! ```
//! use ds_ext::*;
//!
//! let mut set = OrdHashSet::new();
//! assert!(set.is_empty());
//!
//! set.insert(1);
//! assert!(set.contains(&1));
//! assert_eq!(set.len(), 1);
//!
//! let mut map = OrdHashMap::from_iter(set.into_iter().map(|i| (i, i)));
//! assert_eq!(map.len(), 1);
//!
//! assert_eq!(map.get(&1), map.bisect(|i| i.partial_cmp(&1)));
//! ```

#[cfg(feature = "hash")]
mod hash;
#[cfg(feature = "serialize")]
mod serial;
#[cfg(feature = "stream")]
mod stream;

pub mod map;
pub mod queue;
pub mod set;

pub use map::OrdHashMap;
pub use queue::LinkedHashMap;
pub use set::OrdHashSet;

#[cfg(test)]
mod tests {
    #[cfg(feature = "serialize")]
    #[test]
    fn serde_roundtrip_collections() {
        use super::*;
        type MapSetLinked = (
            OrdHashMap<u32, String>,
            OrdHashSet<String>,
            LinkedHashMap<String, u32>,
        );
        let mut map = OrdHashMap::new();
        map.insert(1u32, "one".to_string());
        map.insert(2, "two".to_string());

        let mut set = OrdHashSet::new();
        set.insert("a".to_string());
        set.insert("b".to_string());

        let mut linked = LinkedHashMap::new();
        linked.insert("k1".to_string(), 10u32);
        linked.insert("k2".to_string(), 20u32);

        let empty: MapSetLinked = (OrdHashMap::new(), OrdHashSet::new(), LinkedHashMap::new());
        let single: MapSetLinked = {
            let mut map = OrdHashMap::new();
            map.insert(9, "nine".to_string());
            let mut set = OrdHashSet::new();
            set.insert("z".to_string());
            let mut linked = LinkedHashMap::new();
            linked.insert("only".to_string(), 1u32);
            (map, set, linked)
        };

        let encoded = serde_json::to_string(&(map.clone(), set.clone(), linked.clone()))
            .expect("serialize collections");
        let decoded: MapSetLinked =
            serde_json::from_str(&encoded).expect("deserialize collections");

        assert_eq!(decoded.0, map);
        assert_eq!(decoded.1, set);
        assert_eq!(decoded.2.len(), linked.len());
        for (key, value) in linked.iter() {
            assert_eq!(decoded.2.get(key), Some(value));
        }

        let empty_encoded = serde_json::to_string(&empty).expect("serialize empty collections");
        let empty_decoded: MapSetLinked =
            serde_json::from_str(&empty_encoded).expect("deserialize empty collections");
        assert!(empty_decoded.0.is_empty());
        assert!(empty_decoded.1.is_empty());
        assert!(empty_decoded.2.is_empty());

        let single_encoded = serde_json::to_string(&single).expect("serialize single collections");
        let single_decoded: MapSetLinked =
            serde_json::from_str(&single_encoded).expect("deserialize single collections");
        assert_eq!(single_decoded.0.get(&9).map(|v| v.as_str()), Some("nine"));
        assert!(single_decoded.1.contains(&"z".to_string()));
        assert_eq!(single_decoded.2.get(&"only".to_string()), Some(&1));
    }

    #[cfg(feature = "stream")]
    #[test]
    fn stream_roundtrip_collections() {
        use super::*;
        type MapSetLinked = (
            OrdHashMap<u32, String>,
            OrdHashSet<String>,
            LinkedHashMap<String, u32>,
        );
        let mut map = OrdHashMap::new();
        map.insert(1u32, "one".to_string());
        map.insert(2, "two".to_string());

        let mut set = OrdHashSet::new();
        set.insert("a".to_string());
        set.insert("b".to_string());

        let mut linked = LinkedHashMap::new();
        linked.insert("k1".to_string(), 10u32);
        linked.insert("k2".to_string(), 20u32);

        let empty: MapSetLinked = (OrdHashMap::new(), OrdHashSet::new(), LinkedHashMap::new());
        let single: MapSetLinked = {
            let mut map = OrdHashMap::new();
            map.insert(9, "nine".to_string());
            let mut set = OrdHashSet::new();
            set.insert("z".to_string());
            let mut linked = LinkedHashMap::new();
            linked.insert("only".to_string(), 1u32);
            (map, set, linked)
        };

        let payload = (map.clone(), set.clone(), linked.clone());
        let stream = tbon::en::encode(&payload).expect("encode tbon payload");
        let decoded: MapSetLinked = futures::executor::block_on(tbon::de::try_decode((), stream))
            .expect("decode tbon payload");

        assert_eq!(decoded.0, map);
        assert_eq!(decoded.1, set);
        assert_eq!(decoded.2.len(), linked.len());
        for (key, value) in linked.iter() {
            assert_eq!(decoded.2.get(key), Some(value));
        }

        let empty_stream = tbon::en::encode(&empty).expect("encode empty payload");
        let empty_decoded: MapSetLinked =
            futures::executor::block_on(tbon::de::try_decode((), empty_stream))
                .expect("decode empty payload");
        assert!(empty_decoded.0.is_empty());
        assert!(empty_decoded.1.is_empty());
        assert!(empty_decoded.2.is_empty());

        let single_stream = tbon::en::encode(&single).expect("encode single payload");
        let single_decoded: MapSetLinked =
            futures::executor::block_on(tbon::de::try_decode((), single_stream))
                .expect("decode single payload");
        assert_eq!(single_decoded.0.get(&9).map(|v| v.as_str()), Some("nine"));
        assert!(single_decoded.1.contains(&"z".to_string()));
        assert_eq!(single_decoded.2.get(&"only".to_string()), Some(&1));
    }
}
