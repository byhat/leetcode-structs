use std::cmp::Ord;
use std::collections::BTreeSet;

#[derive(Default, Debug)]
pub struct MultiSet<T> {
    inner: BTreeSet<(T, usize)>,

    counter: usize,
}

impl<T: Ord + Copy> MultiSet<T> {
    pub fn insert(&mut self, val: T) {
        self.inner.insert((val, self.counter));
        self.counter += 1;
    }

    pub fn remove(&mut self, val: &T) -> bool {
        let e = self.inner.range((*val, 0)..(*val, usize::MAX)).next();
        let e = if let Some(inner) = e {
            *inner
        } else {
            return false;
        };

        self.inner.remove(&e);

        true
    }

    pub fn remove_all(&mut self, val: &T) -> usize {
        for i in usize::MIN..usize::MAX {
            if !self.remove(val) {
                return i;
            }
        }

        unreachable!("usize overflowed??")
    }

    pub fn contains(&self, val: &T) -> bool {
        self.inner
            .range((*val, 0)..(*val, usize::MAX))
            .next()
            .is_some()
    }

    pub fn count_range(&self, l: &T, r: &T) -> usize {
        self.inner.range((*l, usize::MIN)..(*r, usize::MIN)).count()
    }

    pub fn first(&self) -> Option<T> {
        self.inner.iter().next().map(|(k, _)| *k)
    }
    pub fn last(&self) -> Option<T> {
        self.inner.iter().next_back().map(|(k, _)| *k)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
