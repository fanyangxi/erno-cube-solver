trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.len()); // capacity? char != byte
        for c in self {
            s.push(c.clone());
        }
        s
    }
}
