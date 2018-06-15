fn filtered_sum() -> i64 {
    (0..1000)
        .filter(|x| x % 3 || x % 5 == 0)
        .sum()
}
