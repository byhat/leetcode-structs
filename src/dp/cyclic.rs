use super::{Memo, Solver};

pub struct CyclicDetector {
    vec: Vec<Vec<usize>>,
    n: usize,
}

impl CyclicDetector {
    pub fn from(vec: Vec<Vec<usize>>) -> Self {
        let n = vec.len();
        Self { vec, n }
    }
}

impl Solver<usize, bool> for CyclicDetector {
    fn exec(&self, memo: &mut impl Memo<usize, bool>, token: usize) -> Option<bool> {
        if token >= self.n {
            return None;
        }

        for i_next in self.vec[token].clone() {
            if !memo.exec(self, i_next).unwrap_or(false) {
                return Some(false);
            }
        }

        Some(true)
    }
}
