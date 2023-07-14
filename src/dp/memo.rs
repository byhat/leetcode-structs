pub trait Memo<I, O> {
    fn exec(&mut self, p: &impl Solver<I, O>, token: I) -> Option<O>;
}

pub trait Solver<I, O> {
    fn exec(&self, memo: &mut impl Memo<I, O>, token: I) -> Option<O>;
}

pub struct Memo1D<T> {
    pending: Vec<bool>,
    inner: Vec<Option<T>>,
}

impl<T: Clone> Memo1D<T> {
    pub fn new(n: usize) -> Self {
        let pending = vec![false; n + 1];
        let inner = vec![None; n + 1];
        Self { pending, inner }
    }
}

impl<T: Clone> Memo<usize, T> for Memo1D<T> {
    fn exec(&mut self, p: &impl Solver<usize, T>, token: usize) -> Option<T> {
        if token >= self.inner.len() {
            return None;
        }
        if self.pending[token] {
            return None;
        }
        if let Some(ret) = self.inner[token].clone() {
            return Some(ret);
        }

        self.pending[token] = true;
        let ret = p.exec(self, token);
        self.pending[token] = false;

        self.inner[token] = ret.clone();

        ret
    }
}
