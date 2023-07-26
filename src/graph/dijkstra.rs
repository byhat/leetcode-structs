use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(graph: &Vec<Vec<(usize, u32)>>, src: usize, n: usize) -> Vec<u64> {
    let mut ret = vec![u64::MAX; n];
    ret[src] = u64::MIN;

    let mut heap = BinaryHeap::new(); // min-heap
    heap.push(Reverse((u64::MIN, src)));

    while let Some(Reverse((dist, vertex))) = heap.pop() {
        if ret[vertex] < dist {
            // a shorter path was revealed
            continue;
        }

        for &(vertex_next, weight) in &graph[vertex] {
            let dist_next = dist + weight as u64;
            if ret[vertex_next] <= dist_next {
                continue;
            }

            ret[vertex_next] = dist_next;
            heap.push(Reverse((dist_next, vertex_next)));
        }
    }

    ret
}
