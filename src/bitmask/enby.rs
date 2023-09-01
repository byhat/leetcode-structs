// A non-binary bitmask serde.
struct EnbyState {
    n: usize,
    threshold: usize,

    op_vec: Vec<usize>,
}

impl EnbyState {
    // Construct a `threshold`-layer bitmask serde with `n` bits.
    pub fn new(n: usize, threshold: usize) -> Self {
        let mut op_vec = Vec::with_capacity(n + 1);
        let mut op = 1usize;
        for _ in 0..=n {
            op_vec.push(op);
            op *= threshold;
        }

        Self {
            n,
            threshold,
            op_vec,
        }
    }

    // Get the `ptr`-th bit of the bitmask `token`.
    pub fn get(&self, mut token: usize, ptr: usize) -> usize {
        assert!(ptr < self.n, "OOB");

        token /= self.op_vec[ptr];
        token %= self.threshold;

        token
    }

    // Set the `ptr`-th bit of the bitmask `token` to `state`.
    pub fn set(&self, mut token: usize, ptr: usize, state: usize) -> usize {
        assert!(ptr < self.n, "OOB");
        assert!(state < self.threshold, "OOT");

        let t0 = token % self.op_vec[ptr];

        token /= self.op_vec[ptr + 1];
        token *= self.threshold;

        token += state;

        token *= self.op_vec[ptr];
        token += t0;

        token
    }

    // Returns the upper bound for this bitmask configuration.
    pub fn upper(&self) -> usize {
        self.op_vec[self.n]
    }
}
