use std::collections::HashMap;

/// The structure of our Key-Value store。
///
/// It persists data by maintaining a `HashMap`
///
/// Data Type:
/// - Key: `String`
/// - Value: `String`
#[derive(Default)]
pub struct KvStore {
    stores: HashMap<String, String>,
}

impl KvStore {
    /// Create an empty `KvStore`
    ///
    /// This `KvStore` will initially create an empty `HashMap` with a capacity of 0,
    /// so it will not allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::store::KvStore;
    /// let kv_store = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            stores: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the `KvStore`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::store::KvStore;
    ///
    /// let mut kv_store = KvStore::new();
    /// kv_store.set("key1".to_owned(), "value1".to_owned());
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.stores.insert(key, value)
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// [`None`]: ../../std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::store::KvStore;
    ///
    /// let mut kv_store = KvStore::new();
    /// kv_store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(kv_store.get(String::from("key1")), Some(String::from("value1")));
    /// assert_eq!(kv_store.get(String::from("key2")), None);
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.stores.get(key.as_str()).cloned()
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::store::KvStore;
    ///
    /// let mut kv_store = KvStore::new();
    /// kv_store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(kv_store.get(String::from("key1")), Some(String::from("value1")));
    ///
    /// kv_store.remove(String::from("key1"));
    /// assert_eq!(kv_store.get(String::from("key1")), None);
    /// ```
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.stores.remove(key.as_str())
    }
}
