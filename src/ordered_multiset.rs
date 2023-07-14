use std::cmp::Ord;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct MultiSet<T> {
    count: usize,
    inner: BTreeMap<T, usize>,
}

impl<T: Ord + Copy> MultiSet<T> {
    pub fn insert(&mut self, val: T) {
        self.inner
            .entry(val)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.count += 1;
    }

    pub fn remove(&mut self, val: &T) -> bool {
        let count = self.inner.remove(val).unwrap_or(0);
        if count < 1 {
            return false;
        }
        if count > 1 {
            self.inner.insert(*val, count - 1);
        }
        self.count -= 1;

        true
    }

    pub fn first(&self) -> Option<T> {
        self.inner.iter().next().map(|(k, _)| *k)
    }
    pub fn last(&self) -> Option<T> {
        self.inner.iter().next_back().map(|(k, _)| *k)
    }

    pub fn pop_first(&mut self) -> Option<T> {
        let e = self.first()?;
        self.remove(&e);

        Some(e)
    }

    pub fn pop_last(&mut self) -> Option<T> {
        let e = self.last()?;
        self.remove(&e);

        Some(e)
    }

    pub fn len(&self) -> usize {
        self.count
    }
}
