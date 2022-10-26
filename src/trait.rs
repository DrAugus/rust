trait AppendStr {
    fn append_bar(self, String: s) -> Self;
}

impl AppendStr for String {
    fn append_bar(mut self, String: s) -> String {
        self.push_str(s);
        self
    }
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self, String: s) -> Vec<String> {
        self.push(s);
        self
    }
}

