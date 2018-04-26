use node::{self, Node};

#[derive(Debug, Clone)]
pub struct PatriciaTree<V> {
    root: Node<V>,
    len: usize,
}
impl<V> PatriciaTree<V> {
    pub fn new() -> Self {
        PatriciaTree {
            root: Node::root(),
            len: 0,
        }
    }
    pub fn into_root(self) -> Node<V> {
        self.root
    }
    pub fn root(&self) -> &Node<V> {
        &self.root
    }
    pub fn insert<K: AsRef<[u8]>>(&mut self, key: K, value: V) -> Option<V> {
        if let Some(old) = self.root.insert(key.as_ref(), value) {
            Some(old)
        } else {
            self.len += 1;
            None
        }
    }
    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<&V> {
        self.root.get(key.as_ref())
    }
    pub fn get_mut<K: AsRef<[u8]>>(&mut self, key: K) -> Option<&mut V> {
        self.root.get_mut(key.as_ref())
    }
    pub fn get_longest_common_prefix<'a>(&self, key: &'a [u8]) -> Option<(&'a [u8], &V)> {
        self.root
            .get_longest_common_prefix(key, 0)
            .map(|(n, v)| (&key[..n], v))
    }
    pub fn remove<K: AsRef<[u8]>>(&mut self, key: K) -> Option<V> {
        if let Some(old) = self.root.remove(key.as_ref()) {
            self.len -= 1;
            Some(old)
        } else {
            None
        }
    }
    pub fn clear(&mut self) {
        self.root = Node::root();
        self.len = 0;
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn nodes(&self) -> Nodes<V> {
        Nodes {
            nodes: self.root.iter(),
            label_lens: Vec::new(),
        }
    }
    pub fn into_nodes(self) -> IntoNodes<V> {
        IntoNodes {
            nodes: self.root.into_iter(),
            label_lens: Vec::new(),
        }
    }
}
impl<V> Default for PatriciaTree<V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<V> From<Node<V>> for PatriciaTree<V> {
    fn from(f: Node<V>) -> Self {
        let mut this = PatriciaTree { root: f, len: 0 };
        let count = this.nodes().count();
        this.len = count;
        this
    }
}

#[derive(Debug)]
pub struct Nodes<'a, V: 'a> {
    nodes: node::Iter<'a, V>,
    label_lens: Vec<usize>,
}
impl<'a, V: 'a> Iterator for Nodes<'a, V> {
    type Item = (usize, &'a Node<V>);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((level, node)) = self.nodes.next() {
            self.label_lens.resize(level + 1, 0);
            self.label_lens[level] = node.label().len();

            let parent_key_len = self.label_lens.iter().take(level).sum();
            Some((parent_key_len, node))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct IntoNodes<V> {
    nodes: node::IntoIter<V>,
    label_lens: Vec<usize>,
}
impl<V> Iterator for IntoNodes<V> {
    type Item = (usize, Node<V>);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((level, node)) = self.nodes.next() {
            self.label_lens.resize(level + 1, 0);
            self.label_lens[level] = node.label().len();

            let parent_key_len = self.label_lens.iter().take(level).sum();
            Some((parent_key_len, node))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = PatriciaTree::new();
        assert_eq!(tree.insert("", 1), None);
        assert_eq!(tree.insert("", 2), Some(1));

        assert_eq!(tree.insert("foo", 3), None);
        assert_eq!(tree.insert("foo", 4), Some(3));

        assert_eq!(tree.insert("foobar", 5), None);

        assert_eq!(tree.insert("bar", 6), None);
        assert_eq!(tree.insert("baz", 7), None);

        assert_eq!(tree.insert("bar", 7), Some(6));
        assert_eq!(tree.insert("baz", 8), Some(7));

        assert_eq!(tree.get(""), Some(&2));
        assert_eq!(tree.get("foo"), Some(&4));
        assert_eq!(tree.get("foobar"), Some(&5));
        assert_eq!(tree.get("bar"), Some(&7));
        assert_eq!(tree.get("baz"), Some(&8));
        assert_eq!(tree.get("qux"), None);

        let tree2 = tree.clone();
        assert_eq!(tree2.get(""), Some(&2));
        assert_eq!(tree2.get("foo"), Some(&4));
        assert_eq!(tree2.get("foobar"), Some(&5));
        assert_eq!(tree2.get("bar"), Some(&7));
        assert_eq!(tree2.get("baz"), Some(&8));

        assert_eq!(tree.remove(""), Some(2));
        assert_eq!(tree.remove("foo"), Some(4));
        assert_eq!(tree.remove("foobar"), Some(5));
        assert_eq!(tree.remove("bar"), Some(7));
        assert_eq!(tree.remove("baz"), Some(8));
        assert_eq!(tree.remove("qux"), None);

        assert_eq!(tree.get(""), None);
        assert_eq!(tree.get("foo"), None);
        assert_eq!(tree.get("foobar"), None);
        assert_eq!(tree.get("bar"), None);
        assert_eq!(tree.get("baz"), None);
        assert_eq!(tree.get("qux"), None);

        assert_eq!(tree2.get(""), Some(&2));
        assert_eq!(tree2.get("foo"), Some(&4));
        assert_eq!(tree2.get("foobar"), Some(&5));
        assert_eq!(tree2.get("bar"), Some(&7));
        assert_eq!(tree2.get("baz"), Some(&8));
    }

}
