use std::collections::{hash_map::Entry, HashMap};

#[derive(Default)]
struct Node {
    children: HashMap<char, Node>,
    is_end: bool,
}

#[derive(Default)]
pub struct Trie {
    root: Node,
    count: i32,
}

#[derive(Debug)]
enum TrieError {
    WordNotFound,
}

impl Trie {
    pub fn count(&self) -> i32 {
        self.count
    }
}

impl Trie {
    pub fn add(&mut self, word: &str) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(Node {
                children: HashMap::new(),
                is_end: false,
            })
        }
        current.is_end = true;
        self.count += 1;
    }
}

impl Trie {
    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            if let Some(found) = current.children.get(&c) {
                current = found;
                continue;
            }
            return false;
        }
        current.is_end
    }
}

impl Trie {
    pub fn delete(&mut self, word: &str) -> Result<(), TrieError> {
        fn delete_recursive(node: &mut Node, word: &str, index: usize) -> Result<bool, TrieError> {
            if index == word.len() {
                if !node.is_end {
                    return Err(TrieError::WordNotFound);
                }
                node.is_end = false;
                return Ok(node.children.is_empty());
            }
            let c = word.chars().nth(index).unwrap();
            match node.children.entry(c) {
                Entry::Occupied(mut entry) => {
                    let next_node = entry.get_mut();
                    let should_delete = delete_recursive(next_node, word, index + 1)?;
                    if should_delete {
                        entry.remove_entry();
                        return Ok(node.children.is_empty() && !node.is_end);
                    };
                }
                Entry::Vacant(_) => return Err(TrieError::WordNotFound),
            }
            Ok(false)
        }
        let result = delete_recursive(&mut self.root, word, 0);
        if result.is_ok() {
            self.count -= 1;
            Ok(())
        } else {
            Err(result.unwrap_err())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_search() {
        let mut trie = Trie::default();
        trie.add("hello");
        trie.add("hey");
        trie.add("hi");

        assert!(trie.search("hello"));
        assert!(trie.search("hey"));
        assert!(trie.search("hi"));
        assert!(!trie.search("test"));
        assert!(!trie.search("he"));
    }

    #[test]
    fn test_delete() {
        let mut trie = Trie::default();
        trie.add("hello");
        trie.add("hey");
        trie.add("hi");

        assert!(trie.search("hello"));
        trie.delete("hello").unwrap();
        assert!(!trie.search("hello"));

        assert!(trie.search("hey"));
        trie.delete("hey").unwrap();
        assert!(!trie.search("hey"));

        assert!(trie.search("hi"));
        trie.delete("hi").unwrap();
        assert!(!trie.search("hi"));

        assert_eq!(trie.count(), 0);
    }

    #[test]
    fn test_delete_non_existing() {
        let mut trie = Trie::default();
        trie.add("hello");

        let result = trie.delete("test");
        assert!(result.is_err());
        match result.unwrap_err() {
            TrieError::WordNotFound => (),
            _ => panic!("Expected WordNotFound error"),
        }
    }

    #[test]
    fn test_delete_prefix() {
        let mut trie = Trie::default();
        trie.add("hello");
        trie.add("hey");

        let result = trie.delete("he");
        assert!(result.is_err());
        match result.unwrap_err() {
            TrieError::WordNotFound => (),
            _ => panic!("Expected WordNotFound error"),
        }
    }
}
