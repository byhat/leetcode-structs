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

// Given a DAG and its adjacency list,
//   returns the vertices sorted in topological order.
pub fn topo_sort_kahn(graph: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = graph.len();

    let mut indeg_vec = vec![usize::MIN; n];
    for v_vec in &graph {
        for &v_next in v_vec {
            indeg_vec[v_next] += 1;
        }
    }

    let mut ret = vec![];

    let mut breadth = vec![];
    for (v, &indeg) in indeg_vec.iter().enumerate() {
        if indeg > 0 {
            continue;
        }
        breadth.push(v);
    }
    while !breadth.is_empty() {
        ret.push(breadth.clone());

        let mut breadth_next = vec![];
        for v in breadth {
            for &v_next in &graph[v] {
                indeg_vec[v_next] -= 1;
                if indeg_vec[v_next] > 0 {
                    continue;
                }
                breadth_next.push(v_next);
            }
        }
        breadth = breadth_next;
    }

    ret
}
