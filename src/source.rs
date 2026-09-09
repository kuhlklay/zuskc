use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub struct Span {
    file_id: usize,
    start: usize,
    len: usize,
}

impl Span {
    pub fn new(start: usize, len: usize, file_id: usize) -> Self {
        Self {
            file_id,
            start,
            len,
        }
    }

    #[inline]
    pub const fn end(&self) -> usize {
        self.start + self.len
    }

    #[inline]
    pub const fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn file_id(&self) -> usize {
        self.file_id
    }

    pub fn as_range(&self) -> std::ops::Range<usize> {
        self.start..self.end()
    }

    pub fn merge(self, other: Span) -> Span {
        assert_eq!(self.file_id, other.file_id);

        let start = self.start.min(other.start);
        let end = self.end().max(other.end());

        Span::new(start, end - start, self.file_id)
    }

    pub fn until(self, other: Span) -> Span {
        assert_eq!(self.file_id, other.file_id);
        assert!(other.start >= self.start);

        Span::new(self.start, other.end() - self.start, self.file_id)
    }
}

pub struct Position {
    pub line: usize,
    pub character: usize,
}

impl Position {
    pub fn new(line: usize, character: usize) -> Self {
        Self { line, character }
    }
}

pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

pub struct SourceFile {
    pub entry: PathBuf,
    pub file_id: usize,
    pub content: String,
    pub line_starts: Vec<usize>,
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

    pub fn position_at(&self, offset: usize) -> Position {
        let line = match self.line_starts.binary_search(&offset) {
            Ok(line) => line,
            Err(line) => line.saturating_sub(1),
        };

        let line_start = self.line_starts[line as usize];

        Position {
            line,
            character: offset - line_start,
        }
    }
}
