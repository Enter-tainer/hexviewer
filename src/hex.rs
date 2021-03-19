pub fn print_hex<T: std::fmt::LowerHex>(v: T) -> String {
    let res = format!("{:0>16x}", v);
    let space = 4;
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
