#[derive(Debug)]
pub struct DisjointSet {
    parents: Vec<Option<usize>>,
    count: usize,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        let parents = vec![None; n];

        Self { parents, count: 0 }
    }

    pub fn init(&mut self, e: usize) {
        if e >= self.parents.len() {
            return;
        }
        if self.parents[e].is_some() {
            return;
        } // already initialized

        self.parents[e] = Some(e);
        self.count += 1; // new representative
    }

    pub fn find(&mut self, e: usize) -> Option<usize> {
        if e >= self.parents.len() {
            return None;
        }

        let parent = self.parents[e]?; // uninitialized
        if parent == e {
            return Some(e);
        } // self-representative

        let ret = self.find(parent)?;
        self.parents[e] = Some(ret); // path compression

        Some(ret)
    }

    pub fn link(&mut self, e0: usize, e1: usize) {
        if let Some(p0) = self.find(e0) {
            if let Some(p1) = self.find(e1) {
                if p0 == p1 {
                    return;
                } // already linked

                // p1 is no longer a representative
                self.parents[p1] = Some(p0);
                self.count -= 1;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        const SET_LEN: usize = 8;

        let mut set = DisjointSet::new(SET_LEN);
        assert_eq!(set.len(), 0);

        for i in 0..SET_LEN {
            set.init(i);
        }
        assert_eq!(set.len(), SET_LEN);
    }

    #[test]
    fn find_uninit() {
        const SET_LEN: usize = 8;

        let mut set = DisjointSet::new(SET_LEN);
        assert!(set.find(2).is_none());
    }

    #[test]
    fn find_linked() {
        const SET_LEN: usize = 8;

        let mut set = DisjointSet::new(SET_LEN);
        for i in 0..SET_LEN {
            set.init(i);
        }
        assert_eq!(set.find(2), Some(2));

        set.link(1, 2);
        assert_eq!(set.find(2), Some(1));

        set.link(0, 2);
        assert_eq!(set.find(1), Some(0));
    }

    #[test]
    fn len() {
        const SET_LEN: usize = 4;

        let mut set = DisjointSet::new(SET_LEN);
        for i in 0..SET_LEN {
            set.init(i);
        }
        assert_eq!(set.len(), SET_LEN);

        set.link(1, 2);
        set.link(0, 2);

        assert_eq!(set.len(), SET_LEN - 2);
    }
}
