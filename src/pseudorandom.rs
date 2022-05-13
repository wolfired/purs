pub fn middle_square_factory(mut seed: usize) -> impl FnMut() -> usize {
    let len = seed.to_string().len();
    let len2 = len * 2;
    let begin = len / 2;
    let end = begin + len;
    move || {
        seed = format!("{:0len2$}", seed * seed)[begin..end]
            .parse::<usize>()
            .unwrap();
        seed
    }
}
