use std::{ops::Range, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub file_id: usize,
    pub start: usize,
    pub len: usize,
}

impl Span {
    pub fn new(start: usize, len: usize, file_id: usize /*, source: SpanSource */) -> Self {
        Self {
            file_id,
            start,
            len,
            // source,
        }
    }

    pub const fn end(self) -> usize {
        self.start + self.len
    }

    pub fn as_range(&self) -> Range<usize> {
        self.start..self.end()
    }

    pub fn merge(self, other: Span) -> Span {
        assert_eq!(self.file_id, other.file_id);

        let start = self.start.min(other.start);
        let end = self.end().max(other.end());

        Span::new(start, end - start, self.file_id)
    }
}

pub struct SourceFile {
    pub entry: PathBuf,
    pub file_id: usize,
    pub content: String,
    line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(entry: PathBuf, file_id: usize) -> Self {
        let content = std::fs::read_to_string(&entry).unwrap();

        let mut line_starts = vec![0];
        for (i, c) in content.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }

        Self {
            entry,
            file_id,
            content,
            line_starts,
        }
    }
}
