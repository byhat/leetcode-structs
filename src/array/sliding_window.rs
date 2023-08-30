pub trait WindowInner<T> {
    fn is_minimum(&self, token_frnt: &T) -> bool;
    fn is_valid(&self) -> bool;

    fn push_back(self, token: T) -> Self;
    fn pop_front(self, token: T) -> Self;
}

use std::collections::VecDeque;

pub struct SlidingWindow<T, I> {
    queue: VecDeque<T>,

    inner: I,
}

impl<T: Copy, I: WindowInner<T>> SlidingWindow<T, I> {
    pub fn new(inner: I) -> Self {
        let queue = VecDeque::new();
        Self { queue, inner }
    }

    pub fn scan(mut self, token: T) -> (Self, Option<Vec<T>>) {
        self.queue.push_back(token);
        self.inner = self.inner.push_back(token);

        while !self.is_minimum() {
            let token_frnt = self.queue.pop_front().unwrap();
            self.inner = self.inner.pop_front(token_frnt);
        }

        let token_out = if self.inner.is_valid() {
            Some(self.queue.clone().into_iter().collect::<Vec<_>>())
        } else {
            None
        };

        (self, token_out)
    }

    fn is_minimum(&self) -> bool {
        if self.queue.is_empty() {
            return true;
        }

        self.inner.is_minimum(self.queue.front().unwrap())
    }
}
