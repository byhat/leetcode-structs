#[derive(Debug, Clone)]
pub struct RollingHasher {
    p: u64,
    m: u64,

    count: usize,

    pow_cache: u64,
    hash: u64,
}

impl RollingHasher {
    pub fn new() -> Self {
        let p = 16807;
        let m = 1_000_000_007;

        let count = usize::MIN;

        let pow_cache = 1u64;
        let hash = u64::MIN;

        Self {
            p,
            m,

            count,

            pow_cache,
            hash,
        }
    }

    pub fn value(&self) -> u64 {
        self.hash
    }
}

impl RollingHasher {
    pub fn push_front(&mut self, token: u64) {
        self.hash *= self.p;
        self.hash += token;
        self.hash %= self.m;

        self.post_trans_hook();
    }

    pub fn push_back(&mut self, token: u64) {
        self.hash += token * self.pow_cache;
        self.hash %= self.m;

        self.post_trans_hook();
    }

    fn post_trans_hook(&mut self) {
        self.count += 1;

        self.pow_cache *= self.p;
        self.pow_cache %= self.m;
    }
}
