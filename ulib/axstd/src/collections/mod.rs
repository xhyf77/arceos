use hashbrown::hash_map as base;
mod random_state;
use random_state::RandomState;
use core::hash::Hash;
use core::fmt::Debug;
use core::fmt;
pub struct Iter<'a, K: 'a, V: 'a> {
    base: base::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
    #[inline]
    fn count(self) -> usize {
        self.base.len()
    }
    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.base.fold(init, f)
    }
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
impl<K, V> Clone for Iter<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Iter { base: self.base.clone() }
    }
}

impl<K: Debug, V: Debug> fmt::Debug for Iter<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}

impl<K, V, S> HashMap<K, V, S> {
    pub const fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap { base: base::HashMap::with_hasher(hash_builder) }
    }
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: self.base.iter() }
    }
}


impl<K, V, S> Default for HashMap<K, V, S>
where
    S: Default,
{
    /// Creates an empty `HashMap<K, V, S>`, with the `Default` value for the hasher.
    #[inline]
    fn default() -> HashMap<K, V, S> {
        HashMap::with_hasher(Default::default())
    }
}


impl<K, V> HashMap<K, V, RandomState> 
where
    K : core::cmp::Eq , 
    K: Hash
{
    #[inline]
    #[must_use]
    pub fn new() -> HashMap<K, V, RandomState> {
        Default::default()
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }

}

impl<'a, K, V, S> IntoIterator for &'a HashMap<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}
