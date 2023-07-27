use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Given a weighted directed graph,
//   returns the minimum distances from `src` to
//   every vertices in the graph.
// Parameters:
//   `graph`: adjacency list for the graph
//   `src`: source vertex
//   `n`: number of vertices
pub fn dijkstra(graph: Vec<Vec<(usize, u32)>>, src: usize, n: usize) -> Vec<Option<u64>> {
    assert!(src < n, "source vertex outside graph");

    let mut ret = vec![None; n];
    ret[src] = Some(u64::MIN);

    let mut heap = BinaryHeap::new(); // min-heap
    heap.push(Reverse((u64::MIN, src)));

    while let Some(Reverse((dist, vertex))) = heap.pop() {
        if ret[vertex].unwrap_or(u64::MAX) < dist {
            // a shorter path was already revealed
            continue;
        }

        for &(vertex_next, weight) in &graph[vertex] {
            let dist_next = dist + weight as u64;
            if ret[vertex_next].unwrap_or(u64::MAX) <= dist_next {
                continue;
            }

            ret[vertex_next] = Some(dist_next);
            heap.push(Reverse((dist_next, vertex_next)));
        }
    }

    ret
}
