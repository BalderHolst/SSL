#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn from_spans(start: &Span, end: &Span) -> Self {
        let start = start.start;
        let end = end.end;
        Self::new(start, end)
    }

    pub fn slice(&self, source: &[u8]) -> String {
        source[self.start..self.end]
            .iter()
            .map(|&b| if b.is_ascii() { b as char } else { '�' })
            .collect()
    }
}
