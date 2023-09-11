use std::collections::HashSet;

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct Prefixes {
    pub prefixes: Vec<HashSet<&'static str>>,
    pub min_len: usize,
    pub max_len: usize,
}

impl Prefixes {
    pub fn new(prefix_list: &[&'static str]) -> Self {
        if prefix_list.is_empty() {
            return Prefixes {
                prefixes: Vec::new(),
                min_len: 0,
                max_len: 0,
            };
        }

        let (mut min_len, mut max_len) = (usize::MAX, usize::MIN);
        for &p in prefix_list {
            let len = p.chars().count();
            debug_assert!(len > 0);
            min_len = min_len.min(len);
            max_len = max_len.max(len);
        }

        let mut prefixes = vec![HashSet::new(); max_len - min_len + 1];
        for &p in prefix_list {
            let len = p.chars().count();
            prefixes[len - min_len].insert(p);
        }

        Prefixes {
            prefixes,
            min_len,
            max_len,
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn contains(&self, prefix: &str) -> bool {
        let len = prefix.chars().count();
        self.contains_hinted(prefix, len)
    }

    #[inline(always)]
    pub fn contains_hinted(&self, prefix: &str, len: usize) -> bool {
        if len < self.min_len || len > self.max_len {
            return false;
        }

        let prefixes = &self.prefixes[len - self.min_len];
        if prefixes.is_empty() {
            return false;
        }

        prefixes.contains(prefix)
    }
}
