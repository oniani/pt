#![warn(clippy::pedantic, missing_docs)]

//! A zero-dependency pure Rust prefix tree optimized for an English alphabet.
//! Current implementation is not space efficient and could be further
//! optimized. One approach is implementing a Patricia tree that groups common
//! prefixes together, ultimately compressing the tree. Another way is to use a
//! clever character encoding technique, which could also reduce the number of
//! buckets. Speed-wise, the current implementation can load over 400, 000
//! words in under 0.3 seconds and thus, is efficient enough for most
//! applications. Searches for words are instantaneous. The downside, however,
//! is that it took over 29, 000, 000 nodes for constructing this prefix tree.

/// `Node` is a type that represents a node for a prefix tree
#[derive(Debug, Default)]
pub struct Node {
    /// Buckets
    pub buckets: [Option<Box<Node>>; 26],
    /// Marker to specify end of word
    pub is_word: bool,
}

/// `PrefixTree` is a type that represents a prefix tree
#[derive(Debug)]
pub struct PrefixTree {
    /// Root of the tree
    pub root: Node,
    /// Number of nodes
    pub num_nodes: u64,
}

impl Default for PrefixTree {
    fn default() -> Self {
        PrefixTree {
            root: Node::default(),
            num_nodes: 26,
        }
    }
}

impl PrefixTree {
    /// `to_index` returns an appropriate index based on character
    ///
    /// # Arguments
    ///
    /// * `char` - A character for which to calculate an index
    fn to_index(c: char) -> usize {
        (c as u8 - 97) as usize
    }

    /// `insert` inserts a word into a prefix tree
    ///
    /// # Arguments
    ///
    /// * `word` - A word to be inserted into a prefix tree
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::default();
    ///
    /// let word = "hello";
    ///
    /// pt.insert(word);
    ///
    /// assert_eq!(pt.search(word), true);
    /// ```
    pub fn insert(&mut self, word: &str) {
        let mut ptr = &mut self.root;

        for idx in word.chars().map(Self::to_index) {
            if ptr.buckets[idx].is_none() {
                self.num_nodes += 26;
                ptr.buckets[idx] = Some(Box::new(Node::default()));
            }
            ptr = ptr.buckets[idx].as_deref_mut().unwrap();
        }

        ptr.is_word = true;
    }

    /// `search` searches for a word in a prefix tree
    ///
    /// # Arguments
    ///
    /// * `word` - A word to be searched in a prefix tree
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::default();
    ///
    /// let word = "hello";
    ///
    /// pt.insert(word);
    ///
    /// assert_eq!(pt.search(word), true);
    /// ```
    pub fn search(&self, word: &str) -> bool {
        let mut ptr = &self.root;

        for idx in word.chars().map(Self::to_index) {
            match &ptr.buckets[idx] {
                Some(bucket) => ptr = bucket,
                None => return false,
            }
        }

        ptr.is_word
    }

    /// `prefix` searches for a prefix word in a prefix tree
    ///
    /// # Arguments
    ///
    /// * `word` - A prefix word to be searched in a prefix tree
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::default();
    ///
    /// let word = "hello";
    /// let is_prefix = "he";
    /// let not_prefix = "ll";
    ///
    /// pt.insert(word);
    ///
    /// assert_eq!(pt.prefix(is_prefix), true);
    /// assert_eq!(pt.prefix(not_prefix), false);
    /// ```
    pub fn prefix(&self, word: &str) -> bool {
        let mut ptr = &self.root;

        for idx in word.chars().map(Self::to_index) {
            match &ptr.buckets[idx] {
                Some(bucket) => ptr = bucket,
                None => return false,
            }
        }

        true
    }

    /// `clear` clears a prefix tree
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::default();
    ///
    /// let word = "hello";
    ///
    /// pt.insert(word);
    ///
    /// assert_eq!(pt.search(word), true);
    ///
    /// pt.clear();
    ///
    /// assert_eq!(pt.search(word), false);
    /// ```
    pub fn clear(&mut self) {
        self.root = Default::default();
    }

    /// `nodes_total` returns a total number of `Node`s in a `PrefixTree`
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::default();
    ///
    /// pt.insert("hello");
    ///
    /// // Total number of nodes is 6 * 26 = 156
    /// assert_eq!(pt.nodes_total(), 156);
    ///
    /// pt.insert("hell");
    ///
    /// // Total number of nodes is the same
    /// assert_eq!(pt.nodes_total(), 156);
    ///
    /// pt.insert("hellicopter");
    ///
    /// // Total number of nodes is 156 + 7 * 26 = 338
    /// assert_eq!(pt.nodes_total(), 338);
    /// ```
    pub fn nodes_total(&self) -> u64 {
        self.num_nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let mut pt = PrefixTree::default();

        let hello = "hello";
        let world = "world";

        pt.insert(hello);
        pt.insert(world);

        assert_eq!(pt.search(hello), true);
        assert_eq!(pt.search(world), true);
        assert_eq!(pt.search("hel"), false);
        assert_eq!(pt.search("orl"), false);

        assert_eq!(pt.prefix("hel"), true);
        assert_eq!(pt.prefix("wor"), true);
        assert_eq!(pt.prefix("elh"), false);
        assert_eq!(pt.prefix("rol"), false);

        pt.clear();

        assert_eq!(pt.search(hello), false);
        assert_eq!(pt.search(world), false);
        assert_eq!(pt.prefix("hel"), false);
        assert_eq!(pt.prefix("wor"), false);
    }

    #[test]
    fn sentence() {
        let mut pt = PrefixTree::default();

        let sentence = "the quick brown fox jumps over the lazy dog";

        for word in sentence.split_whitespace() {
            pt.insert(word);

            assert_eq!(pt.search(word), true);
            assert_eq!(pt.prefix(word), true);
        }
        assert_eq!(pt.nodes_total(), 858);
    }
}
