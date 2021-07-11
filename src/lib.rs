//! A zero-dependency pure Rust prefix tree optimized for an English alphabet.
//! Current implementation is not space efficient and could be further
//! optimized. One approach is implementing a Patricia tree that groups common
//! prefixes together, ultimately compressing the tree. Another way is to use a
//! clever character encoding technique, which could also reduce the number of
//! buckets. Speed-wise, the current implementation can load over 400, 000
//! words in under 0.3 seconds and thus, is efficient enough for most
//! applications. Searches for words are instantaneous. The downside, however,
//! is that it took over 29, 000, 000 nodes for constructing this prefix tree.

#![warn(clippy::all, clippy::pedantic, missing_docs)]

/// `Node` is a type that represents a node for a prefix tree.
#[derive(Debug, Default, PartialEq)]
pub struct Node {
    /// Buckets.
    pub buckets: [Option<Box<Node>>; 26],
    /// Marker to specify end of word.
    pub is_word: bool,
}

/// `PrefixTree` is a type that represents a prefix tree.
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
    /// `new` creates a new prefix tree.
    ///
    /// # Example
    ///
    /// ```
    /// let pt = pt::PrefixTree::new();
    /// dbg!(pt);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// `index` returns an appropriate index based on character.
    ///
    /// # Arguments
    ///
    /// * `char` - A character for which to calculate an index.
    fn index(c: char) -> usize {
        (c as u8 - 97) as usize
    }

    /// `insert` inserts a word into a prefix tree.
    ///
    /// # Arguments
    ///
    /// * `word` - A word to be inserted into a prefix tree.
    ///
    /// # Panics
    ///
    /// This function should never panic.
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::new();
    /// let word = "hello";
    ///
    /// pt.insert(word);
    /// assert_eq!(pt.contains_word(word), true);
    /// ```
    pub fn insert(&mut self, word: &str) {
        let mut ptr = &mut self.root;

        for idx in word.chars().map(Self::index) {
            if ptr.buckets[idx].is_none() {
                self.num_nodes += 26;
                ptr.buckets[idx] = Some(Box::new(Node::default()));
            }

            // SAFETY: This is okay since we know that `ptr.buckets[idx]` is
            // not `None`. In other words, calling `unwrap` on will not result
            // in undefined behavior.
            ptr = ptr.buckets[idx].as_deref_mut().unwrap();
        }

        ptr.is_word = true;
    }

    /// `contains_word` searches for a word in a prefix tree.
    ///
    /// # Arguments
    ///
    /// * `word` - A word to be searched in a prefix tree.
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::new();
    /// let word = "hello";
    ///
    /// pt.insert(word);
    /// assert_eq!(pt.contains_word(word), true);
    /// ```
    #[must_use]
    pub fn contains_word(&self, word: &str) -> bool {
        let mut ptr = &self.root;

        for idx in word.chars().map(Self::index) {
            match &ptr.buckets[idx] {
                Some(bucket) => ptr = bucket,
                None => return false,
            }
        }

        ptr.is_word
    }

    /// `contains_prefix` searches for a prefix word in a prefix tree.
    ///
    /// # Arguments
    ///
    /// * `word` - A prefix word to be searched in a prefix tree.
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::new();
    ///
    /// let word = "hello";
    /// let is_prefix = "he";
    /// let not_prefix = "ll";
    ///
    /// pt.insert(word);
    /// assert_eq!(pt.contains_prefix(is_prefix), true);
    /// assert_eq!(pt.contains_prefix(not_prefix), false);
    /// ```
    #[must_use]
    pub fn contains_prefix(&self, word: &str) -> bool {
        let mut ptr = &self.root;

        for idx in word.chars().map(Self::index) {
            match &ptr.buckets[idx] {
                Some(bucket) => ptr = bucket,
                None => return false,
            }
        }

        true
    }

    /// `nodes_total` returns a total number of `Node`s in a `PrefixTree`.
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::new();
    ///
    /// pt.insert("hello");
    /// assert_eq!(pt.nodes_total(), 156);
    ///
    /// pt.insert("hell");
    /// assert_eq!(pt.nodes_total(), 156);
    ///
    /// pt.insert("hellicopter");
    /// assert_eq!(pt.nodes_total(), 338);
    /// ```
    #[must_use]
    pub fn nodes_total(&self) -> u64 {
        self.num_nodes
    }

    /// `is_empty` checks whether a prefix tree is empty.
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::new();
    /// let word = "bye";
    ///
    /// assert_eq!(pt.is_empty(), true);
    ///
    /// pt.insert(word);
    /// assert_eq!(pt.is_empty(), false);
    /// assert_eq!(pt.contains_word(word), true);
    ///
    /// pt.clear();
    /// assert_eq!(pt.contains_word(word), false);
    /// assert_eq!(pt.is_empty(), true);
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.root == Node::default()
    }

    /// `clear` clears a prefix tree.
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::new();
    /// let word = "hi";
    ///
    /// pt.insert(word);
    /// assert_eq!(pt.contains_word(word), true);
    ///
    /// pt.clear();
    /// assert_eq!(pt.contains_word(word), false);
    /// ```
    pub fn clear(&mut self) {
        self.root = Node::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let mut pt = PrefixTree::new();

        let hello = "hello";
        let world = "world";

        pt.insert(hello);
        pt.insert(world);

        assert_eq!(pt.contains_word(hello), true);
        assert_eq!(pt.contains_word(world), true);
        assert_eq!(pt.contains_word("hel"), false);
        assert_eq!(pt.contains_word("orl"), false);

        assert_eq!(pt.contains_prefix("hel"), true);
        assert_eq!(pt.contains_prefix("wor"), true);
        assert_eq!(pt.contains_prefix("elh"), false);
        assert_eq!(pt.contains_prefix("rol"), false);

        pt.clear();

        assert_eq!(pt.contains_word(hello), false);
        assert_eq!(pt.contains_word(world), false);
        assert_eq!(pt.contains_prefix("hel"), false);
        assert_eq!(pt.contains_prefix("wor"), false);
        assert_eq!(pt.is_empty(), true);
    }

    #[test]
    fn sentence() {
        let mut pt = PrefixTree::new();

        let sentence = "the quick brown fox jumps over the lazy dog";

        for word in sentence.split_whitespace() {
            pt.insert(word);

            assert_eq!(pt.contains_word(word), true);
            assert_eq!(pt.contains_prefix(word), true);
        }
        assert_eq!(pt.nodes_total(), 858);
    }

    #[test]
    fn random_words() {
        let mut pt = PrefixTree::new();

        let words = vec!["afopsiv", "coxpz", "pqeacxnvzm", "zm", "acxk"];

        for word in words {
            pt.insert(word);

            assert_eq!(pt.contains_word(word), true);
            assert_eq!(pt.contains_prefix(word), true);

            for idx in 1..word.len() {
                assert_eq!(pt.contains_prefix(&word[..idx]), true);
                assert_eq!(pt.contains_prefix(&word[idx..]), false);
            }
        }
        assert_eq!(pt.nodes_total(), 728);

        pt.clear();
        assert_eq!(pt.is_empty(), true);
    }
}
