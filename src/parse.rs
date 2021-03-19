pub fn parse_input(str: String) -> String {
    if str.starts_with(":x") {
        let mut res = str.replace(":x", "hex(");
        res.push(')');
        return res;
    }
    if str.starts_with(":b") {
        let mut res = str.replace(":b", "bin(");
        res.push(')');
        return res;
    }
    str
}
