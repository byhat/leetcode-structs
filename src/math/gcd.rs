pub fn gcd(num_0: u32, num_1: u32) -> u32 {
    assert!(num_0 >= num_1, "dxe8=N+");
    if num_0 == num_1 {
        return num_0;
    }
    if num_1 < 2 {
        return num_1;
    }

    let (num_0, num_1) = (num_0 - num_1, num_1);

    gcd(num_0.max(num_1), num_0.min(num_1))
}

use std::collections::BinaryHeap;

pub fn gcd_n_ary(vec: Vec<u32>) -> u32 {
    if vec.is_empty() {
        return u32::MIN;
    }

    let mut heap = BinaryHeap::from(vec);

    while heap.len() > 1 {
        let nums_0 = heap.pop().unwrap(); // always exist
        let nums_1 = heap.pop().unwrap();

        if nums_0 == nums_1 {
            // found gcd of two elements
            heap.push(nums_0);
            continue;
        }

        let (nums_0, nums_1) = (nums_0 - nums_1, nums_1);

        heap.push(nums_0);
        heap.push(nums_1);
    }

    heap.pop().unwrap()
}
