pub fn print_bin<T: std::fmt::Binary>(v: T) -> String {
    let res = format!("{:0>64b}", v);
    let space = 8;
    res.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % space == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}
