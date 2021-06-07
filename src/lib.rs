#![warn(clippy::pedantic, missing_docs)]

//! A zero-dependency pure rust prefix tree optimized for alphabets with at
//! most 32 letters. Current implementation is not space efficient and could be
//! further optimized. One approach is implementing a Patricia Tree that groups
//! common prefixes together, ultimately compressing the tree. Another way is
//! to use a clever character encoding technique, which could also reduce the
//! number of buckets. Speed-wise, the current implementation can load over
//! 400, 000 words in approximately 0.3 seconds and thus, is efficient enough
//! for most applications. Searches for words are instantaneous. The downside,
//! however, is that it took over 29, 000, 000 nodes for constructing this
//! prefix tree.

/// `Node` is a type that represents a node for a prefix tree
#[derive(Debug, Default)]
pub struct Node {
    /// Buckets
    pub buckets: [Option<Box<Node>>; 32],
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
    /// A default implementation of `PrefixTree` contains a default `Node` and
    /// `num_nodes` of 26
    fn default() -> Self {
        PrefixTree {
            root: Node::default(),
            num_nodes: 32,
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
    /// let word = "hello";
    ///
    /// pt.insert(word);
    ///
    /// assert_eq!(pt.search(word), true);
    /// ```
    pub fn insert(&mut self, word: &str) {
        // Get the root pointer
        let mut ptr = &mut self.root;

        // Insert characters into a prefix tree
        for idx in word.chars().map(Self::to_index) {
            // Check for `None` first
            if ptr.buckets[idx].is_none() {
                self.num_nodes += 32;
                ptr.buckets[idx] = Some(Box::new(Node::default()));
            }

            // Dereference is safe since we already check before
            ptr = ptr.buckets[idx].as_deref_mut().unwrap();
        }

        // Mark the end as end of word
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
    /// let word = "hello";
    ///
    /// pt.insert(word);
    ///
    /// assert_eq!(pt.search(word), true);
    /// ```
    pub fn search(&self, word: &str) -> bool {
        // Get the root pointer
        let mut ptr = &self.root;

        // Perform a search for a word
        for idx in word.chars().map(Self::to_index) {
            match &ptr.buckets[idx] {
                Some(bucket) => ptr = bucket,
                None => return false,
            }
        }

        // End of word
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
        // Get the root pointer
        let mut ptr = &self.root;

        // Perform a search for a word
        for idx in word.chars().map(Self::to_index) {
            match &ptr.buckets[idx] {
                Some(bucket) => ptr = bucket,
                None => return false,
            }
        }

        // A word is a prefix if we do not early return
        true
    }

    /// `clear` clears a prefix tree
    ///
    /// # Example
    ///
    /// ```
    /// let mut pt = pt::PrefixTree::default();
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
    /// // Total number of nodes is 6 * 32 = 192
    /// assert_eq!(pt.nodes_total(), 192);
    ///
    /// pt.insert("hell");
    ///
    /// // Total number of nodes is the same
    /// assert_eq!(pt.nodes_total(), 192);
    ///
    /// pt.insert("hellicopter");
    ///
    /// // Total number of nodes is 192 + 7 * 32 = 416
    /// assert_eq!(pt.nodes_total(), 416);
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
        // Initialize a prefix tree
        let mut pt = PrefixTree::default();

        // Dummy words
        let hello = "hello";
        let world = "world";

        // Insert values
        pt.insert(hello);
        pt.insert(world);

        // Test `search`
        assert_eq!(pt.search(hello), true);
        assert_eq!(pt.search(world), true);
        assert_eq!(pt.search("hel"), false);
        assert_eq!(pt.search("orl"), false);

        // Test `prefix`
        assert_eq!(pt.prefix("hel"), true);
        assert_eq!(pt.prefix("wor"), true);
        assert_eq!(pt.prefix("elh"), false);
        assert_eq!(pt.prefix("rol"), false);

        // Test `clear`
        pt.clear();

        assert_eq!(pt.search(hello), false);
        assert_eq!(pt.search(world), false);
        assert_eq!(pt.prefix("hel"), false);
        assert_eq!(pt.prefix("wor"), false);
    }

    #[test]
    fn sentence() {
        // Initialize a prefix tree
        let mut pt = PrefixTree::default();

        // Dummy words
        let sentence = "the quick brown fox jumps over the lazy dog";

        // Insert into a prefix tree and test
        for word in sentence.split_whitespace() {
            pt.insert(word);

            assert_eq!(pt.search(word), true);
            assert_eq!(pt.prefix(word), true);
        }

        // Make sure that total number of nodes is correct
        assert_eq!(pt.nodes_total(), 1056);
    }
}
