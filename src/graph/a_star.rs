use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Given a weighted directed graph,
//   returns the minimum distances from `src` to
//   every vertices in the graph.
// Parameters:
//   `graph`: adjacency list for the graph
//   `src`: source vertex
//   `n`: number of vertices
pub fn a_star<H>(
    graph: Vec<Vec<(usize, u32)>>,
    src: usize,
    dst: usize,
    mut heuristic_fn: H,
) -> Vec<Option<u64>>
where
    H: FnMut(usize) -> u32,
{
    let n = graph.len();

    assert!(src < n, "source vertex outside graph");

    let mut g_vec = vec![None; n];
    g_vec[src] = Some(u64::MIN);

    let src_h = heuristic_fn(src) as u64;
    let mut f_vec = vec![None; n];
    f_vec[src] = Some(src_h);

    let mut heap = BinaryHeap::new(); // min-heap
    heap.push(Reverse((src_h, u64::MIN, src)));

    while let Some(Reverse((f_dist, g_dist, vertex))) = heap.pop() {
        if f_vec[vertex].unwrap_or(u64::MAX) < f_dist {
            // a path with smaller f was already revealed
            continue;
        }

        if vertex == dst {
            break;
        }

        for &(vertex_next, weight) in &graph[vertex] {
            let g_dist_next = g_dist + weight as u64;
            if g_vec[vertex_next].unwrap_or(u64::MAX) <= g_dist_next {
                continue;
            }
            let f_dist_next = g_dist_next + heuristic_fn(vertex_next) as u64;

            g_vec[vertex_next] = Some(g_dist_next);
            f_vec[vertex_next] = Some(f_dist_next);
            heap.push(Reverse((f_dist_next, g_dist_next, vertex_next)));
        }
    }

    g_vec
}
