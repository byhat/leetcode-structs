use std::collections::BinaryHeap;

#[derive(Debug, Default, Clone)]
pub struct MaxHeap<T: std::cmp::Ord + Clone> {
    heap_ent: BinaryHeap<T>,
    heap_ext: BinaryHeap<T>,
}

impl<T: std::cmp::Ord + Clone> MaxHeap<T> {
    pub fn push(&mut self, val: T) {
        self.heap_ent.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        while let Some(e0) = self.heap_ent.pop() {
            while let Some(e1) = self.heap_ext.pop() {
                if e0 >= e1 {
                    self.heap_ext.push(e1);
                    break;
                }
            } // optional

            // All elements in heap_ext <= e0
            if let Some(e1) = self.heap_ext.pop() {
                if e0 <= e1 {
                    continue;
                }
                self.heap_ext.push(e1);
            }

            return Some(e0);
        }

        None
    }

    pub fn top(&mut self) -> Option<T> {
        let val = self.pop()?;
        self.heap_ent.push(val.clone());
        Some(val)
    }

    // val must exist within the max-heap,
    //   otherwise the behavior is undefined.
    pub fn remove(&mut self, val: T) {
        self.heap_ext.push(val);
    }

    pub fn len(&self) -> usize {
        self.heap_ent.len() - self.heap_ext.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        const SET_LEN: usize = 8;

        let mut heap = MaxHeap::default();
        assert_eq!(heap.len(), 0);

        for i in 0..SET_LEN {
            heap.push(i);
        }
        assert_eq!(heap.len(), SET_LEN);
    }

    #[test]
    fn pop() {
        const SET_LEN: usize = 8;

        let mut heap = MaxHeap::default();
        for i in 0..SET_LEN {
            heap.push(i);
        }
        assert_eq!(heap.pop(), Some(SET_LEN - 1));
    }

    #[test]
    fn remove() {
        const SET_LEN: usize = 8;

        let mut heap = MaxHeap::default();
        for i in 0..SET_LEN {
            heap.push(i);
        }
        // [0..SET_LEN]

        heap.remove(SET_LEN - 2);
        assert_eq!(heap.pop(), Some(SET_LEN - 1));
        assert_eq!(heap.pop(), Some(SET_LEN - 3));
        // [0..SET_LEN - 3]

        heap.push(SET_LEN - 3);
        heap.push(SET_LEN - 3);
        heap.push(SET_LEN - 2);
        assert_eq!(heap.pop(), Some(SET_LEN - 2));
        heap.remove(SET_LEN - 3);
        assert_eq!(heap.pop(), Some(SET_LEN - 3));
    }
}
