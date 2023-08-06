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

use std::collections::VecDeque;

// Given a DAG and its adjacency list,
//   returns the vertices sorted in topological order.
pub fn kahn(graph: Vec<Vec<usize>>) -> Vec<usize> {
    let n = graph.len();

    let mut indeg_vec = vec![usize::MIN; n];
    for v_vec in &graph {
        for &v_next in v_vec {
            indeg_vec[v_next] += 1;
        }
    }

    let mut ret = vec![];

    let mut queue = indeg_vec
        .iter()
        .enumerate()
        .filter(|&(_, &e)| e < 1)
        .map(|(i, _)| i)
        .collect::<VecDeque<_>>();
    while let Some(v) = queue.pop_front() {
        ret.push(v);

        for &v_next in &graph[v] {
            indeg_vec[v_next] -= 1;
            if indeg_vec[v_next] < 1 {
                queue.push_back(v_next);
            }
        }
    }

    ret
}
