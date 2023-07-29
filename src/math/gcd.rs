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
