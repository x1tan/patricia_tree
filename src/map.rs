//! A map based on a patricia tree.
use std::fmt;
use std::iter::FromIterator;

use node::Node;
use tree::{self, PatriciaTree};

/// A map based on a patricia tree.
#[derive(Clone)]
pub struct PatriciaMap<V> {
    tree: PatriciaTree<V>,
}
impl<V> PatriciaMap<V> {
    /// Makes a new empty `PatriciaMap` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// assert!(map.is_empty());
    ///
    /// map.insert("foo", 10);
    /// assert_eq!(map.len(), 1);
    /// assert_eq!(map.get("foo"), Some(&10));
    ///
    /// map.remove("foo");
    /// assert_eq!(map.get("foo"), None);
    /// ```
    pub fn new() -> Self {
        PatriciaMap {
            tree: PatriciaTree::new(),
        }
    }

    /// Clears this map, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// map.insert("foo", 1);
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.tree.clear();
    }

    /// Returns `true` if this map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// map.insert("foo", 1);
    /// assert!(map.contains_key("foo"));
    /// assert!(!map.contains_key("bar"));
    /// ```
    pub fn contains_key<K: AsRef<[u8]>>(&self, key: K) -> bool {
        self.tree.get(key).is_some()
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// map.insert("foo", 1);
    /// assert_eq!(map.get("foo"), Some(&1));
    /// assert_eq!(map.get("bar"), None);
    /// ```
    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<&V> {
        self.tree.get(key)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// map.insert("foo", 1);
    /// map.get_mut("foo").map(|v| *v = 2);
    /// assert_eq!(map.get("foo"), Some(&2));
    /// ```
    pub fn get_mut<K: AsRef<[u8]>>(&mut self, key: K) -> Option<&mut V> {
        self.tree.get_mut(key)
    }

    /// Finds the longest common prefix of `key` and the keys in this map,
    /// and returns a reference to the entry whose key matches the prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// map.insert("foo", 1);
    /// map.insert("foobar", 2);
    /// assert_eq!(map.get_longest_common_prefix("fo"), None);
    /// assert_eq!(map.get_longest_common_prefix("foo"), Some(("foo".as_bytes(), &1)));
    /// assert_eq!(map.get_longest_common_prefix("fooba"), Some(("foo".as_bytes(), &1)));
    /// assert_eq!(map.get_longest_common_prefix("foobar"), Some(("foobar".as_bytes(), &2)));
    /// assert_eq!(map.get_longest_common_prefix("foobarbaz"), Some(("foobar".as_bytes(), &2)));
    /// ```
    pub fn get_longest_common_prefix<'a, K>(&self, key: &'a K) -> Option<(&'a [u8], &V)>
    where
        K: AsRef<[u8]> + ?Sized,
    {
        self.tree.get_longest_common_prefix(key.as_ref())
    }

    /// Inserts a key-value pair into this map.
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// assert_eq!(map.insert("foo", 1), None);
    /// assert_eq!(map.get("foo"), Some(&1));
    /// assert_eq!(map.insert("foo", 2), Some(1));
    /// assert_eq!(map.get("foo"), Some(&2));
    /// ```
    pub fn insert<K: AsRef<[u8]>>(&mut self, key: K, value: V) -> Option<V> {
        self.tree.insert(key, value)
    }

    /// Removes a key from this map, returning the value at the key if the key was previously in it.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// map.insert("foo", 1);
    /// assert_eq!(map.remove("foo"), Some(1));
    /// assert_eq!(map.remove("foo"), None);
    /// ```
    pub fn remove<K: AsRef<[u8]>>(&mut self, key: K) -> Option<V> {
        self.tree.remove(key)
    }

    /// Returns the number of elements in this map.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// map.insert("foo", 1);
    /// map.insert("bar", 2);
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    /// Returns `true` if this map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map = PatriciaMap::new();
    /// assert!(map.is_empty());
    ///
    /// map.insert("foo", 1);
    /// assert!(!map.is_empty());
    ///
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets an iterator over the entries of this map, sorted by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let map: PatriciaMap<_> =
    ///     vec![("foo", 1), ("bar", 2), ("baz", 3)].into_iter().collect();
    /// assert_eq!(vec![(Vec::from("bar"), &2), ("baz".into(), &3), ("foo".into(), &1)],
    ///            map.iter().collect::<Vec<_>>());
    /// ```
    pub fn iter(&self) -> Iter<V> {
        Iter {
            nodes: self.tree.nodes(),
            key: Vec::new(),
        }
    }

    /// Gets a mutable iterator over the entries of this map, soretd by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map: PatriciaMap<_> =
    ///     vec![("foo", 1), ("bar", 2), ("baz", 3)].into_iter().collect();
    /// for (_, v) in map.iter_mut() {
    ///    *v += 10;
    /// }
    /// assert_eq!(map.get("bar"), Some(&12));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<V> {
        IterMut {
            nodes: self.tree.nodes(),
            key: Vec::new(),
        }
    }

    /// Gets an iterator over the keys of this map, in sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let map: PatriciaMap<_> =
    ///     vec![("foo", 1), ("bar", 2), ("baz", 3)].into_iter().collect();
    /// assert_eq!(vec![Vec::from("bar"), "baz".into(), "foo".into()],
    ///            map.keys().collect::<Vec<_>>());
    /// ```
    pub fn keys(&self) -> Keys<V> {
        Keys(self.iter())
    }

    /// Gets an iterator over the values of this map, in order by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let map: PatriciaMap<_> =
    ///     vec![("foo", 1), ("bar", 2), ("baz", 3)].into_iter().collect();
    /// assert_eq!(vec![2, 3, 1],
    ///            map.values().cloned().collect::<Vec<_>>());
    /// ```
    pub fn values(&self) -> Values<V> {
        Values {
            nodes: self.tree.nodes(),
        }
    }

    /// Gets a mutable iterator over the values of this map, in order by key.
    ///
    /// # Examples
    ///
    /// ```
    /// use patricia_tree::PatriciaMap;
    ///
    /// let mut map: PatriciaMap<_> =
    ///     vec![("foo", 1), ("bar", 2), ("baz", 3)].into_iter().collect();
    /// for v in map.values_mut() {
    ///     *v += 10;
    /// }
    /// assert_eq!(vec![12, 13, 11],
    ///            map.values().cloned().collect::<Vec<_>>());
    /// ```
    pub fn values_mut(&mut self) -> ValuesMut<V> {
        ValuesMut {
            nodes: self.tree.nodes(),
        }
    }
}
impl<V: fmt::Debug> fmt::Debug for PatriciaMap<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (k, v)) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}: {:?}", k, v)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
impl<V> Default for PatriciaMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<V> IntoIterator for PatriciaMap<V> {
    type Item = (Vec<u8>, V);
    type IntoIter = IntoIter<V>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            nodes: self.tree.into_nodes(),
            key: Vec::new(),
        }
    }
}
impl<K, V> FromIterator<(K, V)> for PatriciaMap<V>
where
    K: AsRef<[u8]>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut map = PatriciaMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}
impl<K, V> Extend<(K, V)> for PatriciaMap<V>
where
    K: AsRef<[u8]>,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}
impl<V> From<Node<V>> for PatriciaMap<V> {
    fn from(f: Node<V>) -> Self {
        PatriciaMap { tree: f.into() }
    }
}
impl<V> From<PatriciaMap<V>> for Node<V> {
    fn from(f: PatriciaMap<V>) -> Self {
        f.tree.into_root()
    }
}
impl<V> AsRef<Node<V>> for PatriciaMap<V> {
    fn as_ref(&self) -> &Node<V> {
        self.tree.root()
    }
}

/// An iterator over a `PatriciaMap`'s entries.
#[derive(Debug)]
pub struct Iter<'a, V: 'a> {
    nodes: tree::Nodes<'a, V>,
    key: Vec<u8>,
}
impl<'a, V: 'a> Iterator for Iter<'a, V> {
    type Item = (Vec<u8>, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key_len, node)) = self.nodes.next() {
            self.key.truncate(key_len);
            self.key.extend(node.label());
            if let Some(value) = node.value() {
                return Some((self.key.clone(), value));
            }
        }
        None
    }
}

/// An owning iterator over a `PatriciaMap`'s entries.
#[derive(Debug)]
pub struct IntoIter<V> {
    nodes: tree::IntoNodes<V>,
    key: Vec<u8>,
}
impl<V> Iterator for IntoIter<V> {
    type Item = (Vec<u8>, V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key_len, mut node)) = self.nodes.next() {
            self.key.truncate(key_len);
            self.key.extend(node.label());
            if let Some(value) = node.take_value() {
                return Some((self.key.clone(), value));
            }
        }
        None
    }
}

/// A mutable iterator over a `PatriciaMap`'s entries.
#[derive(Debug)]
pub struct IterMut<'a, V: 'a> {
    nodes: tree::Nodes<'a, V>,
    key: Vec<u8>,
}
impl<'a, V: 'a> Iterator for IterMut<'a, V> {
    type Item = (Vec<u8>, &'a mut V);
    #[allow(mutable_transmutes)]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key_len, node)) = self.nodes.next() {
            self.key.truncate(key_len);
            self.key.extend(node.label());

            let node = unsafe { &mut *(node as *const _ as *mut Node<V>) };
            if let Some(value) = node.value_mut() {
                return Some((self.key.clone(), value));
            }
        }
        None
    }
}

/// An iterator over a `PatriciaMap`'s keys.
#[derive(Debug)]
pub struct Keys<'a, V: 'a>(Iter<'a, V>);
impl<'a, V: 'a> Iterator for Keys<'a, V> {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, _)| k)
    }
}

/// An iterator over a `PatriciaMap`'s values.
#[derive(Debug)]
pub struct Values<'a, V: 'a> {
    nodes: tree::Nodes<'a, V>,
}
impl<'a, V: 'a> Iterator for Values<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_, node)) = self.nodes.next() {
            if let Some(value) = node.value() {
                return Some(value);
            }
        }
        None
    }
}

/// A mutable iterator over a `PatriciaMap`'s values.
#[derive(Debug)]
pub struct ValuesMut<'a, V: 'a> {
    nodes: tree::Nodes<'a, V>,
}
impl<'a, V: 'a> Iterator for ValuesMut<'a, V> {
    type Item = &'a mut V;
    #[allow(mutable_transmutes)]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_, node)) = self.nodes.next() {
            let node = unsafe { &mut *(node as *const _ as *mut Node<V>) };
            if let Some(value) = node.value_mut() {
                return Some(value);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use rand::{self, Rng};
    use super::*;

    #[test]
    fn it_works() {
        let input = [
            ("7", 7),
            ("43", 43),
            ("92", 92),
            ("37", 37),
            ("31", 31),
            ("21", 21),
            ("0", 0),
            ("35", 35),
            ("47", 47),
            ("82", 82),
            ("61", 61),
            ("9", 9),
        ];

        let mut map = PatriciaMap::new();
        for &(ref k, v) in input.iter() {
            assert_eq!(map.insert(k, v), None);
            assert_eq!(map.get(k), Some(&v));
        }
    }

    #[test]
    fn debug_works() {
        let map: PatriciaMap<_> = vec![("foo", 1), ("bar", 2), ("baz", 3)]
            .into_iter()
            .collect();
        assert_eq!(
            format!("{:?}", map),
            "{[98, 97, 114]: 2, [98, 97, 122]: 3, [102, 111, 111]: 1}"
        );
    }

    #[test]
    fn clear_works() {
        let mut map = PatriciaMap::new();
        assert!(map.is_empty());

        map.insert("foo", 1);
        assert!(!map.is_empty());

        map.clear();
        assert!(map.is_empty());
    }

    #[test]
    fn into_iter_works() {
        let map: PatriciaMap<_> = vec![("foo", 1), ("bar", 2), ("baz", 3)]
            .into_iter()
            .collect();
        assert_eq!(
            map.into_iter().collect::<Vec<_>>(),
            [(Vec::from("bar"), 2), ("baz".into(), 3), ("foo".into(), 1)]
        );
    }

    #[test]
    fn large_map_works() {
        let mut input = (0..10000).map(|i| (i.to_string(), i)).collect::<Vec<_>>();
        rand::thread_rng().shuffle(&mut input[..]);

        // Insert
        let mut map = input.iter().cloned().collect::<PatriciaMap<_>>();
        assert_eq!(map.len(), input.len());

        // Get
        for &(ref k, v) in input.iter() {
            assert_eq!(map.get(k), Some(&v));
        }

        // Remove
        for &(ref k, v) in input.iter().take(input.len() / 2) {
            assert_eq!(map.remove(k), Some(v));
            assert_eq!(map.remove(k), None);
        }
        for &(ref k, _) in input.iter().take(input.len() / 2) {
            assert_eq!(map.get(k), None);
        }
        for &(ref k, v) in input.iter().skip(input.len() / 2) {
            assert_eq!(map.get(k), Some(&v));
        }

        // Insert
        for &(ref k, v) in input.iter().take(input.len() / 2) {
            assert_eq!(map.insert(k, v), None);
        }
        for &(ref k, v) in input.iter().skip(input.len() / 2) {
            assert_eq!(map.insert(k, v), Some(v));
        }

        // Get
        for &(ref k, v) in input.iter() {
            assert_eq!(map.get(k), Some(&v));
        }
    }
}
