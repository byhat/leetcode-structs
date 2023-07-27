use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn prim(graph: Vec<Vec<(usize, u32)>>, n: usize) -> Vec<(usize, usize, u32)> {
    if graph.is_empty() {
        return vec![];
    }

    let mut heap = BinaryHeap::new();
    for &(v_next, weight) in &graph[usize::MIN] {
        heap.push(Reverse((weight, v_next, usize::MIN)));
    }

    let mut ret = vec![];

    let mut visited = vec![false; n];
    visited[usize::MIN] = true;
    while let Some(Reverse((weight, v, v_prev))) = heap.pop() {
        if visited[v] {
            continue;
        }
        visited[v] = true;

        ret.push((v_prev, v, weight));

        for &(v_next, weight_next) in &graph[v] {
            heap.push(Reverse((weight_next, v_next, v)));
        }
    }

    ret
}
