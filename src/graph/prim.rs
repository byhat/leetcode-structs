use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn prim(graph: Vec<Vec<(usize, u32)>>, n: usize) -> Vec<(usize, usize, u32)> {
    if graph.is_empty() {
        return vec![];
    }

    let mut ret = vec![];

    let mut heap = BinaryHeap::new();
    for &(v_next, weight) in &graph[usize::MIN] {
        heap.push(Reverse((weight, v_next, usize::MIN)));
    }

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

pub fn prim_with_edge(
    graph: Vec<Vec<(usize, u32)>>,
    edge: (usize, usize, u32),
    n: usize,
) -> Vec<(usize, usize, u32)> {
    if graph.is_empty() {
        return vec![];
    }

    let (v0, v1, weight) = edge;
    let mut ret = vec![(v0, v1, weight)];

    let mut heap = BinaryHeap::new();
    for &(v_next, weight) in &graph[v0] {
        heap.push(Reverse((weight, v_next, v0)));
    }
    for &(v_next, weight) in &graph[v1] {
        heap.push(Reverse((weight, v_next, v1)));
    }

    let mut visited = vec![false; n];
    visited[v0] = true;
    visited[v1] = true;
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
