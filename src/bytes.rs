use std::{slice::Iter, iter::Enumerate};

const NULL: u8 = b'\0';
const SPACE: u8 = b' ';
const HORIZONTAL_TAB: u8 = b'\t';
const LINE_FEED: u8 = b'\n';
const VERTICAL_TAB: u8 = b'\x0B';
const FORM_FEED: u8 = b'\x0C';
const CARRIAGE_RETURN: u8 = b'\r';

struct Lines<'a> {
    slice: &'a [u8],
    bytes: Enumerate<Iter<'a, u8>>,
    index: usize,
}

impl<'a> Lines<'a> {
    fn new(slice: &'a [u8]) -> Self {
        Self {
            slice,
            bytes: slice.into_iter().enumerate(),
            index: 0,
        }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.slice.len() {
            return None;
        }

        let line_start_i = self.index;
        let mut line_end_i = line_start_i;
        let mut next_i = line_start_i + 1;
        let mut previous_i = 0;
        let mut previous_byte = NULL;

        while let Some((current_i, &current_byte)) = self.bytes.next() {
            next_i = current_i + 1;

            if current_byte == LINE_FEED {
                line_end_i =
                    if previous_byte == CARRIAGE_RETURN {
                        previous_i
                    } else {
                        current_i
                    };

                break;
            }

            line_end_i = next_i;
            previous_i = current_i;
            previous_byte = current_byte;
        }

        self.index = next_i;
        let line = &self.slice[line_start_i..line_end_i];

        Some(line)
    }
}

fn is_whitespace(byte: u8) -> bool {
    match byte {
        SPACE |
        HORIZONTAL_TAB |
        LINE_FEED |
        VERTICAL_TAB |
        FORM_FEED |
        CARRIAGE_RETURN => true,
        _ => false,
    }
}

pub trait Bytes {
    fn lines(&self) -> impl Iterator<Item = &[u8]>;

    fn trim(&self) -> &[u8];
}

impl Bytes for [u8] {
    fn lines(&self) -> impl Iterator<Item = &[u8]> {
        Lines::new(self)
    }

    fn trim(&self) -> &[u8] {
        let mut start = 0;
        for (i, &byte) in self.iter().enumerate() {
            if !is_whitespace(byte) {
                start = i;
                break;
            }
        };

        let mut end = self.len();
        for &byte in self.iter().rev() {
            if !is_whitespace(byte) {
                break;
            }

            end -= 1;
        }

        &self[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_of_empty() {
        assert_eq!(b"".lines().next(), None);
    }

    #[test]
    fn lines_of_no_line_breaks() {
        let bytes = b"hello";
        let mut lines = bytes.lines();

        assert_eq!(lines.next(), Some(b"hello".as_slice()));
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn lines_of_just_line_break() {
        let bytes = b"\n";
        let mut lines = bytes.lines();

        assert_eq!(lines.next(), Some(&[][..]));
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn lines_with_mixed_line_breaks() {
        let bytes = b"alpha\nbeta\r\ngamma\rdelta\r\nepsilon\nzeta";
        let mut lines = bytes.lines();

        assert_eq!(lines.next(), Some(b"alpha".as_slice()));
        assert_eq!(lines.next(), Some(b"beta".as_slice()));
        assert_eq!(lines.next(), Some(b"gamma\rdelta".as_slice()));
        assert_eq!(lines.next(), Some(b"epsilon".as_slice()));
        assert_eq!(lines.next(), Some(b"zeta".as_slice()));
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn lines_with_mixed_line_breaks_and_trailing_line_break() {
        let bytes = b"alpha\nbeta\r\ngamma\r\rdelta\r\nepsilon\nzeta\r\n";
        let mut lines = bytes.lines();

        assert_eq!(lines.next(), Some(b"alpha".as_slice()));
        assert_eq!(lines.next(), Some(b"beta".as_slice()));
        assert_eq!(lines.next(), Some(b"gamma\r\rdelta".as_slice()));
        assert_eq!(lines.next(), Some(b"epsilon".as_slice()));
        assert_eq!(lines.next(), Some(b"zeta".as_slice()));
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn trim_empty() {
        let bytes = b"";
        assert_eq!(bytes.trim(), bytes.as_slice());
    }

    #[test]
    fn trim_nothing() {
        let bytes = b"hello";
        assert_eq!(bytes.trim(), bytes.as_slice());
    }

    #[test]
    fn trim_left() {
        let bytes = [
            SPACE,
            SPACE,
            VERTICAL_TAB,
            FORM_FEED,
            HORIZONTAL_TAB,
            CARRIAGE_RETURN,
            LINE_FEED,
            b'a',
            b'b',
            b'c',
        ];
        assert_eq!(bytes.trim(), b"abc");
    }

    #[test]
    fn trim_right() {
        let bytes = [
            b'a',
            b'b',
            b'c',
            SPACE,
            SPACE,
            VERTICAL_TAB,
            FORM_FEED,
            HORIZONTAL_TAB,
            CARRIAGE_RETURN,
            LINE_FEED,
        ];
        assert_eq!(bytes.trim(), b"abc");
    }

    #[test]
    fn trim_both_side() {
        let bytes = [
            SPACE,
            SPACE,
            VERTICAL_TAB,
            FORM_FEED,
            HORIZONTAL_TAB,
            CARRIAGE_RETURN,
            LINE_FEED,
            b'a',
            b'b',
            b'c',
            SPACE,
            SPACE,
            VERTICAL_TAB,
            FORM_FEED,
            HORIZONTAL_TAB,
            CARRIAGE_RETURN,
            LINE_FEED,
        ];
        assert_eq!(bytes.trim(), b"abc");
    }
}
