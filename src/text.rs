use crate::style::Style;
use std::{borrow::Cow, cmp::max};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Span<'a> {
    pub content: Cow<'a, str>,
    pub style: Style,
}

impl<'a> Span<'a> {
    pub fn raw<T>(content: T) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn styled<T>(content: T, style: Style) -> Span<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Span {
            content: content.into(),
            style,
        }
    }

    pub fn width(&self) -> usize {
        self.content.width()
    }
}

impl<'a> From<String> for Span<'a> {
    fn from(s: String) -> Span<'a> {
        Span::raw(s)
    }
}

impl<'a> From<&'a str> for Span<'a> {
    fn from(s: &'a str) -> Span<'a> {
        Span::raw(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line<'a> {
    pub spans: Vec<Span<'a>>,
}

impl<'a> Default for Line<'a> {
    fn default() -> Line<'a> {
        Line { spans: Vec::new() }
    }
}

impl<'a> From<String> for Line<'a> {
    fn from(s: String) -> Line<'a> {
        Line::with_spans(vec![Span::from(s)])
    }
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Line<'a> {
        Line::with_spans(vec![Span::from(s)])
    }
}

impl<'a> From<Vec<Span<'a>>> for Line<'a> {
    fn from(spans: Vec<Span<'a>>) -> Line<'a> {
        Line { spans }
    }
}

impl<'a> From<Span<'a>> for Line<'a> {
    fn from(span: Span<'a>) -> Line<'a> {
        Line { spans: vec![span] }
    }
}

impl<'a> Line<'a> {
    pub fn with_spans(spans: Vec<Span<'a>>) -> Line<'a> {
        Line { spans }
    }

    pub fn width(&self) -> usize {
        self.spans.iter().fold(0, |acc, s| acc + s.width())
    }
}

impl<'a> From<Line<'a>> for String {
    fn from(line: Line<'a>) -> String {
        line.spans.iter().fold(String::new(), |mut acc, s| {
            acc.push_str(s.content.as_ref());
            acc
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text<'a> {
    lines: Vec<Line<'a>>,
}

impl<'a> Default for Text<'a> {
    fn default() -> Text<'a> {
        Text { lines: Vec::new() }
    }
}

impl<'a> From<Vec<Line<'a>>> for Text<'a> {
    fn from(lines: Vec<Line<'a>>) -> Text<'a> {
        Text { lines }
    }
}

impl<'a> Text<'a> {
    pub fn with_lines<T>(lines: T) -> Text<'a>
    where
        T: Into<Vec<Line<'a>>>,
    {
        Text {
            lines: lines.into(),
        }
    }

    pub fn width(&self) -> usize {
        self.lines.iter().fold(0, |acc, l| max(acc, l.width()))
    }
}
