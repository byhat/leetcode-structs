pub struct RabinKarpSearch {
    p: u64,
    m: u64,

    n: usize,

    hash_vec: Vec<u64>,
    pow_vec: Vec<u64>,
}

impl RabinKarpSearch {
    pub fn new(vec: Vec<u64>) -> Self {
        let p = 16807;
        let m = 1_000_000_007;

        let n = vec.len();

        let mut hash_vec = Vec::with_capacity(n + 1);

        let mut hash = u64::MIN;
        hash_vec.push(hash);
        for token in vec {
            hash *= p;
            hash += token;
            hash %= m;

            hash_vec.push(hash);
        }

        let mut pow_vec = Vec::with_capacity(n + 1);

        let mut op = 1u64;
        pow_vec.push(op);
        for _ in 0..n {
            op *= p;
            op %= m;

            pow_vec.push(op);
        }

        Self {
            p,
            m,

            n,

            hash_vec,
            pow_vec,
        }
    }
}

impl RabinKarpSearch {
    pub fn query(&self, p0: usize, p1: usize) -> u64 {
        if p0 >= p1 {
            return u64::MIN;
        }
        if p1 > self.n {
            return u64::MIN;
        }

        let mut hash_1 = self.hash_vec[p1];
        hash_1 += self.m;

        let mut hash_0 = self.hash_vec[p0];
        hash_0 *= self.pow_vec[p1 - p0];
        hash_0 %= self.m;

        (hash_1 - hash_0) % self.m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let n = 10usize;
        let vec = (0..10).map(|e| e as u64 + 1).collect::<Vec<_>>();

        let segment = RabinKarpSearch::new(vec);
        assert_eq!(segment.query(0, 0), u64::MIN);
        assert_ne!(segment.query(0, n - 1), segment.query(1, n));
    }

    #[test]
    fn eq() {
        let n = 10usize;
        let vec = vec![1, 2, 3, 4, 5, 1, 2, 3, 5];

        let segment = RabinKarpSearch::new(vec);
        assert_eq!(segment.query(0, 3), segment.query(5, 8));
        assert_ne!(segment.query(0, 4), segment.query(5, 9));
    }
}
