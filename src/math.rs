pub fn ipow(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1;
    while exp > 0 {
        if exp & 1 > 0 {
            result *= base;
        }
        exp >>= 1;
        base *= base;
    }

    result
}
