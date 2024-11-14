/// A span of text in the source code.
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a new span from a start and end index.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Create a new span from a start span and end span.
    pub fn from_spans(start: &Span, end: &Span) -> Self {
        let start = start.start;
        let end = end.end;
        Self::new(start, end)
    }

    /// Get the string that the span points to in the source.
    pub fn get_string(&self, source: &[u8]) -> String {
        source[self.start..self.end]
            .iter()
            .map(|&b| if b.is_ascii() { b as char } else { '�' })
            .collect()
    }
}
