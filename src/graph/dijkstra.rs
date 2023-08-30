use std::cmp::Reverse;
use std::collections::BTreeSet;
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

// Given a weighted directed graph,
//   returns the `k`-th minimum distances from `src` to
//   every vertices in the graph.
// Parameters:
//   `graph`: adjacency list for the graph
//   `src`: source vertex
//   `n`: number of vertices
//   `k`: the number of minimum distances to track
//
// TODO: UNTESTED!!
pub fn dijkstra_kth(
    graph: Vec<Vec<(usize, u32)>>,
    src: usize,
    n: usize,
    k: usize,
) -> Vec<Vec<u64>> {
    assert!(src < n, "source vertex outside graph");

    let mut ret = vec![BinaryHeap::new(); n]; // max-heap
    ret[src].push(u64::MIN);

    let mut heap = BinaryHeap::new(); // min-heap
    heap.push(Reverse((u64::MIN, src)));

    while let Some(Reverse((dist, vertex))) = heap.pop() {
        for &(vertex_next, weight) in &graph[vertex] {
            let dist_next = dist + weight as u64;
            let ret_heap = &mut ret[vertex_next];

            // TODO: is this necessary??
            if ret_heap.len() >= k {
                if let Some(dist_max) = ret_heap.peek().cloned() {
                    if dist_max <= dist_next {
                        continue; // heap is full, abort
                    }
                }
            }

            ret_heap.push(dist_next);
            while ret_heap.len() > k {
                ret_heap.pop();
            }

            heap.push(Reverse((dist_next, vertex_next)));
        }
    }

    ret.into_iter()
        .map(|v| {
            let mut v = v.into_iter().collect::<Vec<_>>();
            v.sort_unstable();
            v
        })
        .collect::<Vec<_>>()
}

// Given a weighted directed graph,
//   returns the `k`-th minimum unique distances from `src` to
//   every vertices in the graph.
// Parameters:
//   `graph`: adjacency list for the graph
//   `src`: source vertex
//   `n`: number of vertices
//   `k`: the number of minimum distances to track
pub fn dijkstra_kth_unique(
    graph: Vec<Vec<(usize, u32)>>,
    src: usize,
    n: usize,
    k: usize,
) -> Vec<Vec<u64>> {
    assert!(src < n, "source vertex outside graph");

    let mut ret = vec![BTreeSet::new(); n]; // max-heap
    ret[src].insert(u64::MIN);

    let mut heap = BinaryHeap::new(); // min-heap
    heap.push(Reverse((u64::MIN, src)));

    while let Some(Reverse((dist, vertex))) = heap.pop() {
        for &(vertex_next, weight) in &graph[vertex] {
            let dist_next = dist + weight as u64;
            let ret_set = &mut ret[vertex_next];
            if ret_set.contains(&dist_next) {
                continue; // already reached
            }

            if ret_set.len() >= k {
                if let Some(dist_max) = ret_set.iter().rev().next().cloned() {
                    if dist_max <= dist_next {
                        continue; // heap is full, abort
                    }
                }
            }

            ret_set.insert(dist_next);
            while ret_set.len() > k {
                let dist_max = ret_set.iter().rev().next().cloned().unwrap();
                ret_set.remove(&dist_max);
            }

            heap.push(Reverse((dist_next, vertex_next)));
        }
    }

    ret.into_iter()
        .map(|v| {
            let mut v = v.into_iter().collect::<Vec<_>>();
            v.sort_unstable();
            v
        })
        .collect::<Vec<_>>()
}
