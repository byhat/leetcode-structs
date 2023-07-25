pub fn nCr(n: usize, r: usize) -> usize {
    let mut res = 1usize;

    let mut div = 2usize;
    let div_end = n - r + 1;

    for i in (r + 1)..=n {
        res *= i;

        while div < div_end && res % div == 0 {
            res /= div;
            div += 1;
        }
    }

    res
}
