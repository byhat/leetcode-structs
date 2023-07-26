use crate::dp::{Memo, Solver};

pub struct InDegree {
    vec: Vec<Vec<usize>>,
    n: usize,
}

impl InDegree {
    pub fn from(vec: Vec<Vec<usize>>) -> Self {
        let n = vec.len();
        Self { vec, n }
    }
}

impl Solver<usize, usize> for InDegree {
    fn exec(&self, memo: &mut impl Memo<usize, usize>, token: usize) -> Option<usize> {
        if token >= self.n {
            return None;
        }

        let mut ret = usize::MIN;
        for i_next in self.vec[token].clone() {
            ret = ret.max(memo.exec(self, i_next)? + 1);
        }

        Some(ret)
    }
}
